
#[macro_use]
extern crate lazy_static;
extern crate mut_static;

mod Functionality;
mod Structures;

use std::fs;

use Structures::Instruction::Instruction;
use Functionality::{Preprocess, CodeGen, TagResolution};
fn main() {
    
    //initialize the tag_resolutor
    Functionality::TagResolution::TAGDICT.set(std::collections::HashMap::new()).unwrap();

    let fs = fs::read_to_string("irqh.s").unwrap();

    let mut instr_v: Vec<((String, usize),Option<Instruction>)> = Vec::new();
    let mut line_count = 1;
    let original_addr: u32 = 0x40000000;
    let mut addr: u32 = original_addr;

    //preprocessing
    for (original_line, processed) in Preprocess::replacement(fs) {
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
            Preprocess::LineTag::Label(l) => {
                instr = None;
                TagResolution::log_addr(l, addr);
            }
        }

        instr_v.push(((original_line, line_count),instr));

        line_count += 1;
    }

    //compilation
    let mut addr = original_addr;
    for ((original_line, line_count), instr_maybe) in instr_v {
        print!("{}\t| {} ",line_count, original_line);
        match instr_maybe {
            Some(instr) => {
                let c: u32 = CodeGen::compile(instr, addr);
                println!(" -> 0X{:08X}", c);
                addr += 0x4;
            }
            None => {
                println!();
            }
        }
    }
}
