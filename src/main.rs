#[macro_use]
extern crate lazy_static;
extern crate clap;
extern crate mut_static;
extern crate structure;
extern crate core;

#[allow(non_snake_case)]
mod Functionality;
#[allow(non_snake_case)]
mod Structures;

use clap::Parser;
use std::fs;
use std::process::exit;

use Functionality::{CodeGen, Preprocess, TagResolution};
use Structures::Data::Data;
use Structures::Instruction::Instruction;
use Structures::RELFHeaders::Sections;

#[derive(Parser, Debug)]
#[clap(
    author = "Axemt <github.com/Axemt>",
    version = "0.5 built on Jun 5, 2022",
    about = "A MIPS R3000 32b compiler",
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
    let code_digest = Preprocess::digest(&fs);

    let mut addr: u32 = 0; //a segment start has to be processed before any instructions
    let mut instr_v: Vec<((String, usize), Option<Instruction>)> = Vec::new();
    let mut data_v: Vec<((String, usize), Option<Data>)> = Vec::new();

    let mut is_code_segment = false;
    let mut code_base_addr = 0;
    let mut data_base_addr = 0;

    let mut line_count = 1;
    //preprocessing
    for (original_line, processed) in code_digest {
        let mut instr: Option<Instruction> = None;
        let mut data: Option<Data> = None;
        //dbg!(&processed);
        match processed {
            Preprocess::LineTag::Processed(l) => {
                if is_code_segment {
                    instr = Some(l.into())
                };

                addr += 0x4;
            }
            Preprocess::LineTag::Ignore => {
                instr = None;
            }
            Preprocess::LineTag::Tag(tag) => {
                instr = None;
                //are we on data? if so, they are categorized as tag
                if !is_code_segment {
                    data = Some(String::from(&original_line).into())
                }
                TagResolution::log_addr(tag, addr);
            }
            Preprocess::LineTag::SectionStart(a, section_type) => {
                addr = a;
                match section_type {
                    Sections::Code => {
                        is_code_segment = true;
                        code_base_addr = a;
                    }
                    Sections::Data => {
                        is_code_segment = false;
                        data_base_addr = a;
                    }
                    //_ => is_code_segment = false,
                }
            }
        }

        if is_code_segment {
            instr_v.push(((original_line, line_count), instr));
        } else {
            data_v.push(((original_line, line_count), data))
        }
        line_count += 1;
    }

    //compilation
    let (code, data) = match CodeGen::compile(code_base_addr, instr_v, data_base_addr, data_v) {
        Ok((c, v)) => (c, v),
        Err(e) => {
            eprintln!("{}", e);
            exit(-1)
        }
    };

    CodeGen::pack_and_write(args.output, code_base_addr, code, data_base_addr, data);
}
