use super::{
    Instruction::InstructionType,
    Opcodes::OPCODES::{I, J, R},
};

use crate::Functionality::TagResolution;
use crate::Functionality::TagResolution::Tag;

use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct ArgumentBundle {
    pub rs: u32,
    pub rt: u32,
    pub rd: u32,
    pub sham: u32,
    pub imm: Tag, //for jump instructions, imm is used as jtarg
}

impl Default for ArgumentBundle {
    fn default() -> Self {
        Self {
            rs: Default::default(),
            rt: Default::default(),
            rd: Default::default(),
            sham: Default::default(),
            imm: Tag::Imm(0),
        }
    }
}

impl ArgumentBundle {
    pub fn construct(arg_vec: VecDeque<&str>, itype: &InstructionType, func: u32) -> Self {
        match itype {
            InstructionType::I => ArgumentBundle::construct_I(arg_vec, func),
            InstructionType::R => ArgumentBundle::construct_R(arg_vec, func),
            InstructionType::J => ArgumentBundle::construct_J(arg_vec, func),
            InstructionType::Special => {
                panic!("A special-type instruction reached ArgumentBundle construct");
            }
        }
    }

    fn construct_R(mut arg_vec: VecDeque<&str>, func: u32) -> Self {
        // rd first except in mult, div and jr where it is rs
        let mut rd;
        let mut rs;
        let mut rt;
        let mut sham;
        match func {
            R::MULT | R::MULTU | R::DIV | R::DIVU | R::JR => {
                // <op> rs ..
                rs = arg_vec.pop_front().unwrap().parse().unwrap();

                //then if it is JR -> we're done
                //else the second is rt
                if func != R::JR {
                    rt = arg_vec.pop_front().unwrap().parse().unwrap()
                } else {
                    rt = 0
                }

                return ArgumentBundle {
                    rs,
                    rt,
                    rd: 0,
                    sham: 0,
                    imm: Tag::Imm(0),
                };
            }
            _ => {
                // <op> rd rs ...
                rd = arg_vec.pop_front().unwrap().parse().unwrap();
                rs = arg_vec.pop_front().unwrap().parse().unwrap();

                if func == R::MFHI || func == R::MFLO {
                    return ArgumentBundle {
                        rs,
                        rt: 0,
                        rd,
                        sham: 0,
                        imm: Tag::Imm(0),
                    };
                };

                //rt at the end for all except shifts

                match func {
                    R::SLL | R::SRA | R::SRAV | R::SRLV => {
                        sham = arg_vec.pop_front().unwrap().parse().unwrap();
                        rt = 0;
                    }

                    _ => {
                        sham = 0;
                        rt = arg_vec.pop_front().unwrap().parse().unwrap();
                    }
                }

                ArgumentBundle {
                    rs,
                    rt,
                    rd,
                    sham,
                    imm: Tag::Imm(0),
                }
            }
        }
    }

    fn construct_I(mut arg_vec: VecDeque<&str>, func: u32) -> Self {
        //tags are assumed to have been converted to the actual jump target

        let rs;
        let rt;
        let imm;
        //dbg!(&arg_vec);

        match func {
            I::BGTZ | I::BLEZ => {
                //conditional jumps: <op> rs, imm
                rs = arg_vec.pop_front().unwrap().parse().unwrap();

                let imm_candidate = arg_vec.pop_front().unwrap();
                imm = if imm_candidate
                    .matches(char::is_numeric)
                    .collect::<String>()
                    .len()
                    == imm_candidate.len()
                {
                    //all characters were numbers, it is a proper imm
                    Tag::Imm(imm_candidate.parse().unwrap())
                } else {
                    TagResolution::log_or_resolve(imm_candidate)
                };
                rt = 0;
            }
            I::BNE | I::BEQ => {
                //bne: <bne> rs rt imm
                rs = arg_vec.pop_front().unwrap().parse().unwrap();
                rt = arg_vec.pop_front().unwrap().parse().unwrap();
                let imm_candidate = arg_vec.pop_front().unwrap();
                imm = if imm_candidate
                    .matches(char::is_numeric)
                    .collect::<String>()
                    .len()
                    == imm_candidate.len()
                {
                    //all characters were numbers, it is a proper imm
                    Tag::Imm(imm_candidate.parse().unwrap())
                } else {
                    TagResolution::log_or_resolve(imm_candidate)
                };
            }
            _ => {
                // other: <op> rt, rs, imm
                rt = arg_vec.pop_front().unwrap().parse().unwrap();
                rs = arg_vec.pop_front().unwrap().parse().unwrap();
                let imm_candidate: String = arg_vec.pop_front().unwrap().to_string();
                imm = Tag::Imm(if imm_candidate.starts_with("0X") || imm_candidate.starts_with("0x") {
                    u32::from_str_radix(imm_candidate.trim_start_matches("0x").trim_start_matches("0X"),16).unwrap()
                } else {
                    imm_candidate.parse().unwrap()
                });
            }
        }

        ArgumentBundle {
            rs,
            rt,
            rd: 0,
            sham: 0,
            imm,
        }
    }

    fn construct_J(mut arg_vec: VecDeque<&str>, _func: u32) -> Self {
        let imm_candidate = arg_vec.pop_front().unwrap();
        let imm = if imm_candidate.matches(char::is_numeric).collect::<String>().len() == imm_candidate.len() {
            //all characters were numbers, it is a proper imm
            Tag::Imm(imm_candidate.parse().unwrap())
        } else {
            TagResolution::log_or_resolve(imm_candidate)
        };

        ArgumentBundle {
            rs: 0,
            rt: 0,
            rd: 0,
            sham: 0,
            imm,
        }
    }
}
