use crate::Structures::Data::{DType, Data};
use crate::Structures::Instruction::{Instruction, InstructionType};
use crate::Structures::RELFHeaders::{RelfHeader32, SectionHeader32};

use super::TagResolution;
use super::TagResolution::Tag;

use crate::Structures::Errors::CompileError;

use structure::*;

use std::fs::File;
use std::io::Write;

pub fn pack_and_write(
    path: String,
    code_base_addr: u32,
    code: Vec<u32>,
    data_base_addr: u32,
    data: Vec<u8>,
) {
    let mut relf_header = RelfHeader32::default();
    relf_header.e_entry = code_base_addr;

    let mut program_header = SectionHeader32::default();
    program_header.p_type = 0x00000001;
    program_header.p_offset = (relf_header.e_phentsize * 2) as u32;
    program_header.p_vaddr = code_base_addr;
    program_header.p_paddr = code_base_addr;
    program_header.p_filesz = code.len() as u32 * 4;
    program_header.p_memsz = code.len() as u32 * 4;
    program_header.p_flags = 0x05000000;

    let data_header = SectionHeader32 {
        p_type: 0x00000001,
        p_offset: relf_header.e_phentsize as u32,
        p_vaddr: data_base_addr,
        p_paddr: data_base_addr,
        p_filesz: data.len() as u32 * 4,
        p_memsz: data.len() as u32 * 4,
        p_flags: 0x06000000,
        ..Default::default()
    };

    {
        let mut fd = File::create(path).expect("Could not create the output file");
        let elf_header_format = structure!(">IBBBBB7sHHIIIIIHHHHHH");
        let pd_header_format = structure!(">IIIIIIII");

        let elf_header = elf_header_format
            .pack(
                relf_header.e_ident_MAG,
                relf_header.e_ident_CLASS,
                relf_header.e_ident_DATA,
                relf_header.e_ident_VERSION,
                relf_header.e_ident_OSABI,
                relf_header.e_ident_ABIVERSION,
                &relf_header.e_ident_EIPAD,
                relf_header.e_type,
                relf_header.e_machine,
                relf_header.e_version,
                relf_header.e_entry,
                relf_header.e_phoff,
                relf_header.e_shoff,
                relf_header.e_flags,
                relf_header.e_ehsize,
                relf_header.e_phentsize,
                relf_header.e_phnum,
                relf_header.e_shentsize,
                relf_header.e_shnum,
                relf_header.e_shstrndx,
            )
            .unwrap();

        fd.write_all(&elf_header).expect("Could not write to file");

        let text_header_p = pd_header_format
            .pack(
                program_header.p_type,
                program_header.p_offset,
                program_header.p_vaddr,
                program_header.p_paddr,
                program_header.p_filesz,
                program_header.p_memsz,
                program_header.p_flags,
                program_header.p_align,
            )
            .unwrap();

        fd.write_all(&text_header_p)
            .expect("Could not write to file");

        let data_header_p = pd_header_format
            .pack(
                data_header.p_type,
                data_header.p_offset,
                data_header.p_vaddr,
                data_header.p_paddr,
                data_header.p_filesz,
                data_header.p_memsz,
                data_header.p_flags,
                data_header.p_align,
            )
            .unwrap();

        fd.write_all(&data_header_p)
            .expect("Could not write to file");

        for c in code {
            fd.write(&c.to_be_bytes()).expect("Could not write to file");
        }

        // data is already an u8 buffer :)
        fd.write(&data).expect("Could not write to file");
    }
}

pub fn compile(
    code_base_addr: u32,
    instr_v: Vec<((String, usize), Option<Instruction>)>,
    data_base_addr: u32,
    data_v: Vec<((String, usize), Option<Data>)>,
) -> Result<(Vec<u32>, Vec<u8>), CompileError> {
    let mut d_addr = data_base_addr;
    let mut data: Vec<u8> = Vec::new(); // dense array of contiguous data. We do not allow hosting data at different mem locations for now

    for ((original_line, line_count), data_maybe) in data_v {
        print!("{}\t| {} ", line_count, original_line);
        match data_maybe {
            Some(dt) => {
                //ensure alignment
                match dt.dt {
                    DType::Word => {
                        if d_addr % 4 != 0 {
                            return Err(CompileError::AlignmentError(4, d_addr, dt.tagname));
                        }
                    }
                    DType::Half => {
                        if d_addr % 2 != 0 {
                            return Err(CompileError::AlignmentError(2, d_addr, dt.tagname));
                        }
                    }
                    DType::Byte | DType::ZTerminatedString | DType::String | DType::Space => {}
                }

                TagResolution::log_addr(dt.tagname, d_addr);

                print!(" -> 0x");
                let mut print_ct = 0;
                for byte in &dt.contents {
                    print!("{:02X}", byte);
                    print_ct += 1;
                    if print_ct > 4 {
                        break;
                    }
                }
                if dt.contents.len() > 4 {
                    print!("...")
                };

                println!(" @ [0x{:08X}]", d_addr);
                d_addr += dt.contents.len() as u32;
                for byte in dt.contents {
                    data.push(byte);
                }
            }
            None => {
                println!();
            }
        }
    }

    if code_base_addr % 2 != 0 {
        return Err(CompileError::AlignmentError(
            2,
            code_base_addr,
            String::from(".text"),
        ));
    }

    let mut c_addr = code_base_addr;
    let mut code: Vec<u32> = Vec::new();

    for ((original_line, line_count), instr_maybe) in instr_v {
        print!("{}\t| {} ", line_count, original_line);
        match instr_maybe {
            Some(instr) => {
                let compiled: u32 = compile_single(instr, c_addr)?;
                println!(" -> 0x{:08X} @ [0x{:08X}]", compiled, c_addr);
                c_addr += 0x4;
                code.push(compiled);
            }
            None => {
                println!();
            }
        }
    }

    //ensure no overlaps between segments
    //code is either after the data segment or before it
    assert!(
        ((code_base_addr >= data_base_addr + data.len() as u32)
            || (code.len() as u32 * 4 + code_base_addr < data_base_addr)),
        "Code and Data segments overlap!"
    );

    Ok((code, data))
}

fn compile_single(instr: Instruction, addr: u32) -> Result<u32, CompileError> {
    //
    // See https://uweb.engr.arizona.edu/~ece369/Resources/spim/MIPSReference.pdf for sources on encoding formats
    // Not all instructions are implemented but including them now saves future time
    //
    // See https://www.eg.bucknell.edu/~csci320/mips_web/ for checking the correctness of the encoding

    if instr.args.rs > 32 {
        return Err(CompileError::RegisterParse("RS".into()));
    }
    if instr.args.rd > 32 {
        return Err(CompileError::RegisterParse("RD".into()));
    }
    if instr.args.rt > 32 {
        return Err(CompileError::RegisterParse("RT".into()));
    }

    Ok(match instr.itype {
        InstructionType::I => compile_I(instr, addr)?,
        InstructionType::R => compile_R(instr)?,
        InstructionType::J => compile_J(instr)?,
        InstructionType::Special => instr.func,
    })
}

fn compile_I(instr: Instruction, addr: u32) -> Result<u32, CompileError> {
    let imm = match instr.args.imm {
        Tag::Imm(v, imm_sign_negative) => {
            if imm_sign_negative {
                v | 0b00000000000000001000000000000000
            } else {
                v
            }
        }
        Tag::Resolved(tagaddr) => {
            if tagaddr < addr {
                //since tagaddr and addr are u32, we need to force them to fit to 16b with the bitmask
                ((tagaddr.overflowing_sub(addr).0 - 4) >> 2) & 0x0000ffff
            } else {
                (tagaddr - addr - 4) >> 2
            }
        }
        Tag::BuildPending(s) => {
            let tagaddr = TagResolution::resolve(s)?;
            if tagaddr < addr {
                ((tagaddr.overflowing_sub(addr).0 - 4) >> 2) & 0x0000ffff
            } else {
                (tagaddr - addr - 4) >> 2
            }
        }
    };

    if imm > 65536 {
        return Err(CompileError::ImmSize);
    };
    let func_c = (instr.func << 26) & 0b11111100000000000000000000000000;
    let rs_c = (instr.args.rs << 21) & 0b00000011111000000000000000000000;
    let rt_c = (instr.args.rt << 16) & 0b00000000000111110000000000000000;
    let imm_c = imm & 0b00000000000000001111111111111111;

    Ok(func_c | rs_c | rt_c | imm_c)
}

fn compile_R(instr: Instruction) -> Result<u32, CompileError> {
    let rs_c = (instr.args.rs << 21) & 0b00000011111000000000000000000000;
    let rt_c = (instr.args.rt << 16) & 0b00000000000111110000000000000000;
    let rd_c = (instr.args.rd << 11) & 0b00000000000000001111100000000000;
    let sham_c = (instr.args.sham << 6) & 0b00000000000000000000011111000000;
    let func_c = instr.func & 0b00000000000000000000000000111111;

    Ok(rs_c | rt_c | rd_c | sham_c | func_c)
}

fn compile_J(instr: Instruction) -> Result<u32, CompileError> {
    let jtarg = match instr.args.imm {
        Tag::Imm(v, imm_sign_negative) => {
            if imm_sign_negative {
                v | 0b00000000000000001000000000000000
            } else {
                v
            }
        }
        Tag::Resolved(v) => v,
        Tag::BuildPending(s) => TagResolution::resolve(s)?,
    };

    let func_c = (instr.func << 26) & 0xfc000000;
    let jtarg_c = (jtarg & !0xfc000000) >> 2;
    Ok(func_c | jtarg_c)
}

#[test]
fn compile_label_data_resolution() {
    TagResolution::init();
    let d = String::from("some_tag: .word 1,2,3,4");
    let i = String::from("j some_tag");
    dbg!(&d);
    dbg!(&i);

    const ADDR_CODE: u32 = 0x00000000;
    const ADDR_DATA: u32 = 0x0000ff00;
    let (code, data) = match compile(
        ADDR_CODE,
        vec![((i.clone(), 1), Some(i.into()))],
        ADDR_DATA,
        vec![((d.clone(), 1), Some(d.into()))],
    ) {
        Ok((c, d)) => (c, d),
        Err(eobj) => {
            panic!("{}", eobj)
        }
    };
    println!("{:08X}", (code[0] & !0xfc000000) << 2);
    assert!((code[0] & !0xfc000000) << 2 == ADDR_DATA);
}

#[test]
#[should_panic]
fn label_data_non_aligned() {
    TagResolution::init();
    let d = String::from("some_tag: .word 1,2,3,4");
    let i = String::from("j some_tag");
    dbg!(&d);
    dbg!(&i);

    const ADDR_CODE: u32 = 0x00000000;
    const ADDR_DATA: u32 = 0x0000ffdd;
    let (code, data) = match compile(
        ADDR_CODE,
        vec![((i.clone(), 1), Some(i.into()))],
        ADDR_DATA,
        vec![((d.clone(), 1), Some(d.into()))],
    ) {
        Ok((c, d)) => (c, d),
        Err(eobj) => {
            panic!("{}", eobj)
        }
    };
}
