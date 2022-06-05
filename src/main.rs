#[macro_use]
extern crate lazy_static;
extern crate clap;
extern crate mut_static;
extern crate structure;

mod Functionality;
mod Structures;


use clap::Parser;
use structure::*;
use core::panic;
use std::fs;
use std::fs::File;
use std::io::Write;

use Functionality::{CodeGen, Preprocess, TagResolution};
use Structures::Instruction::Instruction;
use Structures::RELFHeaders::{RelfHeader32, SectionHeader32};

#[derive(Parser, Debug)]
#[clap(
    author = "Axemt <github.com/Axemt>",
    version = "0.92 built on Feb 21, 2022",
    about = "A MIPS R3000 32b emulator",
    long_about = None
)]
struct Args {
    #[clap(short = 'i', long = "input", help = "File to compile", required = true)]
    input: String,
    #[clap(
        short = 'o',
        long = "output",
        help = "File to write to",
        required = true
    )]
    output: String,
}

fn main() {
    let args = Args::parse();

    //initialize the tag_resolutor
    Functionality::TagResolution::init();

    let fs = fs::read_to_string(args.input).unwrap();
    let (metadata, code_digest) = Preprocess::digest(&fs);

    if !metadata.contains_key(".text") {
        panic!("\".text\" segment not defined for this file!");
    }

    let code_base_addr: u32 = metadata[".text"];
    if !code_base_addr % 4 == 0 {
        panic!("Base address for \".text\" segment is not word-aligned");
    }

    let data_base_addr = if metadata.contains_key(".data") {
        let data_base_addr_candidate = metadata[".data"];
        if !data_base_addr_candidate % 4 == 0 {
            panic!("Base address for \".data\" segment is not word-aligned");
        }
        data_base_addr_candidate
    } else {
        0
    };

    let mut addr: u32 = code_base_addr;
    let mut instr_v: Vec<((String, usize), Option<Instruction>)> = Vec::new();
    let mut line_count = 1;

    //preprocessing
    for (original_line, processed) in code_digest {
        let instr: Option<Instruction>;
        //dbg!(&processed);
        match processed {
            Preprocess::LineTag::Processed(l) => {
                instr = Some(l.into());

                addr += 0x4;
            }
            Preprocess::LineTag::Ignore => {
                instr = None;
            }
            Preprocess::LineTag::Tag(tag) => {
                instr = None;
                TagResolution::log_addr(tag, addr);
            }
        }

        instr_v.push(((original_line, line_count), instr));

        line_count += 1;
    }

    //compilation
    let mut addr = code_base_addr;
    let mut code: Vec<u32> = Vec::new();
    for ((original_line, line_count), instr_maybe) in instr_v {
        print!("{}\t| {} ", line_count, original_line);
        match instr_maybe {
            Some(instr) => {
                let c: u32 = CodeGen::compile(instr, addr);
                println!(" -> 0X{:08X} @ [0X{:08X}]", c, addr);
                addr += 0x4;
                code.push(c);
            }
            None => {
                println!();
            }
        }
    }
    
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

    let mut data_header = SectionHeader32::default();
    data_header.p_type = 0x00000001;
    data_header.p_offset = relf_header.e_phentsize as u32;
    data_header.p_vaddr = 0; //TODO
    data_header.p_paddr = data_base_addr;
    data_header.p_filesz = 0; // TODO data.len() as u32 * 4
    data_header.p_memsz = 0; // TODO data.len() as u32 * 4
    data_header.p_flags = 0x06000000;

    {
        let mut fd = File::create(args.output).expect("Could not create the output file");
        let ELFheaderFormat = structure!(">IBBBBB7sHHIIIIIHHHHHH");
        let pdHeaderFormat = structure!(">IIIIIIII");

        let elfHeader = ELFheaderFormat.pack(
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
            relf_header.e_shstrndx
        ).unwrap();

        fd.write_all(&elfHeader);

        let text_header_p = pdHeaderFormat.pack(
            program_header.p_type, 
            program_header.p_offset, 
            program_header.p_vaddr,
            program_header.p_paddr,
            program_header.p_filesz, 
            program_header.p_memsz,
            program_header.p_flags,
            program_header.p_align
        ).unwrap();

        fd.write_all(&text_header_p);

        let data_header_p = pdHeaderFormat.pack(
            data_header.p_type, 
            data_header.p_offset, 
            data_header.p_vaddr,
            data_header.p_paddr,
            data_header.p_filesz, 
            data_header.p_memsz,
            data_header.p_flags,
            data_header.p_align
        ).unwrap();

        fd.write_all(&data_header_p);

        for c in code {
            fd.write(&c.to_be_bytes());
        }

        //Nothing regarding data atm






    }


}
