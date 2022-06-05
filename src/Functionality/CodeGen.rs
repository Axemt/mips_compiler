use crate::Structures::{
    ArgumentBundle,
    Instruction::{Instruction, InstructionType},
};

use super::TagResolution;
use super::TagResolution::Tag;

pub fn compile(instr: Instruction, addr: u32) -> u32 {
    //
    // See https://uweb.engr.arizona.edu/~ece369/Resources/spim/MIPSReference.pdf for sources on encoding formats
    // Not all instructions are implemented but including them now saves future time
    //
    // See https://www.eg.bucknell.edu/~csci320/mips_web/ for checking the correctness of the encoding
    assert!(
        instr.args.rs < 32 && instr.args.rt < 32 && instr.args.rd < 32,
        "Error parsing registers"
    );
    match instr.itype {
        InstructionType::I => compile_I(instr, addr),
        InstructionType::R => compile_R(instr),
        InstructionType::J => compile_J(instr),
        InstructionType::Special => instr.func,
    }
}

fn compile_I(instr: Instruction, addr: u32) -> u32 {
    let imm_sign_negative = false;
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
            let tagaddr = TagResolution::resolve(s);
            if tagaddr < addr {
                ((tagaddr.overflowing_sub(addr).0 - 4) >> 2) & 0x0000ffff
            } else {
                (tagaddr - addr - 4) >> 2
            }
        }
    };

    assert!(imm < 65536, "Given immediate does not fit in 16b");
    let func_c = (instr.func << 26) & 0b11111100000000000000000000000000;
    let rs_c = (instr.args.rs << 21) & 0b00000011111000000000000000000000;
    let rt_c = (instr.args.rt << 16) & 0b00000000000111110000000000000000;
    let mut imm_c = imm & 0b00000000000000001111111111111111;

    func_c | rs_c | rt_c | imm_c
}

fn compile_R(instr: Instruction) -> u32 {
    let rs_c = (instr.args.rs << 21) & 0b00000011111000000000000000000000;
    let rt_c = (instr.args.rt << 16) & 0b00000000000111110000000000000000;
    let rd_c = (instr.args.rd << 11) & 0b00000000000000001111100000000000;
    let sham_c = (instr.args.sham << 6) & 0b00000000000000000000011111000000;
    let func_c = instr.func & 0b00000000000000000000000000111111;

    rs_c | rt_c | rd_c | sham_c | func_c
}

fn compile_J(instr: Instruction) -> u32 {
    let imm_sign_negative = false;
    let jtarg = match instr.args.imm {
        Tag::Imm(v, imm_sign_negative) => {
            if imm_sign_negative {
                v | 0b00000000000000001000000000000000
            } else {
                v
            }
        }
        Tag::Resolved(v) => v,
        Tag::BuildPending(s) => TagResolution::resolve(s),
    };

    let func_c = (instr.func << 26) & 0xfc000000;
    let jtarg_c = (jtarg & !0xfc000000) >> 2;
    func_c | jtarg_c
}
