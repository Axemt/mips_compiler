
use std::collections::VecDeque;

use super::ArgumentBundle::ArgumentBundle;
use super::Opcodes::OPCODES;


#[derive(PartialEq, Debug)]
pub enum InstructionType {
    I,
    R,
    J,
    Special
}

#[derive(Debug)]
pub struct Instruction {
    pub itype: InstructionType,
    pub func: u32,
    pub args: ArgumentBundle,
}

impl From<String> for Instruction {
    fn from(st: String) -> Self {
        //Parse the string
        let mut tokens: VecDeque<&str> = st.split(|c: char| if c == ' '  || c == ',' {true} else {false} ).collect();
        //1. Figure out OP type
        let op = tokens.pop_front().unwrap();
        let (func, itype) = match_func(op);

        //1.1: If itype == special, func describes the entire instruction and we return early
        if itype == InstructionType::Special {
            return Instruction { itype, func, args: ArgumentBundle::default() };
        }
        //2. Construct appropiate ArgumentBundle

        let args = ArgumentBundle::construct(tokens, &itype, func);

        //??
        //Profit!

        Instruction { itype, func, args }

    }
}

impl From<&str> for Instruction {
    fn from(st: &str) -> Self {
        st.to_string().into()
    }
}

fn match_func(op: &str) -> (u32, InstructionType) {

    match op {

    "add"   => { (OPCODES::R::ADD,   InstructionType::R) }
    "addu"  => { (OPCODES::R::ADDU,  InstructionType::R) }
    "and"   => { (OPCODES::R::AND,   InstructionType::R) }
    "nor"   => { (OPCODES::R::NOR,   InstructionType::R) }
    "or"    => { (OPCODES::R::OR,    InstructionType::R) }
    "sub"   => { (OPCODES::R::SUB,   InstructionType::R) }
    "subu"  => { (OPCODES::R::SUBU,  InstructionType::R) }
    "xor"   => { (OPCODES::R::XOR,   InstructionType::R) }
    "slt"   => { (OPCODES::R::SLT,   InstructionType::R) }
    "sltu"  => { (OPCODES::R::SLTU,  InstructionType::R) }
    "div"   => { (OPCODES::R::DIV,   InstructionType::R) }
    "divu"  => { (OPCODES::R::DIVU,  InstructionType::R) }
    "mult"  => { (OPCODES::R::MULT,  InstructionType::R) }
    "multu" => { (OPCODES::R::MULTU, InstructionType::R) }
    "sll"   => { (OPCODES::R::SLL,   InstructionType::R) }
    "sra"   => { (OPCODES::R::SRA,   InstructionType::R) }
    "srav"  => { (OPCODES::R::SRAV,  InstructionType::R) }
    "srlv"  => { (OPCODES::R::SRLV,  InstructionType::R) }
    "jarl"  => { (OPCODES::R::JARL,  InstructionType::R) }
    "jr"    => { (OPCODES::R::JR,    InstructionType::R) }
    "mfhi"  => { (OPCODES::R::MFHI,  InstructionType::R) }
    "mflo"  => { (OPCODES::R::MFLO,  InstructionType::R) }
    "mthi"  => { (OPCODES::R::MTHI,  InstructionType::R) }
    "mtlo"  => { (OPCODES::R::MTLO,  InstructionType::R) }

    
    "addi"  => { (OPCODES::I::ADDI,  InstructionType::I) }
    "addiu" => { (OPCODES::I::ADDIU, InstructionType::I) }
    "andi"  => { (OPCODES::I::ANDI,  InstructionType::I) }
    "ori"   => { (OPCODES::I::ORI,   InstructionType::I) }
    "xori"  => { (OPCODES::I::XORI,  InstructionType::I) }
    "slti"  => { (OPCODES::I::SLTI,  InstructionType::I) }
    "sltiu" => { (OPCODES::I::SLTIU, InstructionType::I) }
    "lhi"   => { (OPCODES::I::LHI,   InstructionType::I) }
    "llo"   => { (OPCODES::I::LLO,   InstructionType::I) }
    "beq"   => { (OPCODES::I::BEQ,   InstructionType::I) }
    "bne"   => { (OPCODES::I::BNE,   InstructionType::I) }
    "bgtz"  => { (OPCODES::I::BGTZ,  InstructionType::I) }
    "blez"  => { (OPCODES::I::BLEZ,  InstructionType::I) }
    "lb"    => { (OPCODES::I::LB,    InstructionType::I) }
    "lbu"   => { (OPCODES::I::LBU,   InstructionType::I) }
    "lh"    => { (OPCODES::I::LH,    InstructionType::I) }
    "lhu"   => { (OPCODES::I::LHU,   InstructionType::I) }
    "lw"    => { (OPCODES::I::LW,    InstructionType::I) }
    "sb"    => { (OPCODES::I::SB,    InstructionType::I) }
    "sh"    => { (OPCODES::I::SH,    InstructionType::I) }
    "sw"    => { (OPCODES::I::SW,    InstructionType::I) }

    "jal"   => { (OPCODES::J::JAL,   InstructionType::J) }
    "j"     => { (OPCODES::J::J,     InstructionType::J) }


    "hlt"   => { (OPCODES::HLT,      InstructionType::Special) }
    "rfe"   => { (OPCODES::RFE,      InstructionType::Special) }
    "nop"   => { (OPCODES::HLT,      InstructionType::Special) }
    "syscall" => { (OPCODES::SYSCALL,InstructionType::Special) }

    _ => { panic!("Unrecognized opcode {}", op); }

    }
}

#[test]
fn conversion() {
    let instr: Instruction = "addi 1,2,3".into();
    dbg!(instr);
}