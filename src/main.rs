#[macro_use]
extern crate lazy_static;
extern crate mut_static;

mod Functionality;
mod Structures;

use core::panic;
use std::fs;

use Functionality::{
    CodeGen, Preprocess,
    TagResolution::{self, TAGDICT},
};
use Structures::Instruction::Instruction;
fn main() {
    //initialize the tag_resolutor
    Functionality::TagResolution::init();

    let fs = fs::read_to_string("irqh.s").unwrap();
    let metadata = Preprocess::find_segment_metadata(&fs);

    if !metadata.contains_key(".text") {
        panic!("\".text\" segment not defined for this file!");
    }

    let code_base_addr: u32 = metadata[".text"];
    if !code_base_addr % 4 == 0 {
        panic!("Base address for \".text\" segment is not word-aligned");
    }

    let mut addr: u32 = code_base_addr;
    let mut instr_v: Vec<((String, usize), Option<Instruction>)> = Vec::new();
    let mut line_count = 1;

    //preprocessing
    for (original_line, processed) in Preprocess::digest(&fs) {
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
    for ((original_line, line_count), instr_maybe) in instr_v {
        print!("{}\t| {} ", line_count, original_line);
        match instr_maybe {
            Some(instr) => {
                let c: u32 = CodeGen::compile(instr, addr);
                println!(" -> 0X{:08X} @ [0X{:08X}]", c, addr);
                addr += 0x4;
            }
            None => {
                println!();
            }
        }
    }
}
