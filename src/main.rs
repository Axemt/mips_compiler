#[macro_use]
extern crate lazy_static;
extern crate clap;
extern crate mut_static;
extern crate structure;

#[allow(non_snake_case)]
mod Functionality;
#[allow(non_snake_case)]
mod Structures;

use clap::Parser;
use std::fs;
use std::process::exit;

use Functionality::{CodeGen, Preprocess, TagResolution};
use Structures::Instruction::Instruction;

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
    let (metadata, code_digest) = Preprocess::digest(&fs);

    let (code_base_addr, data_base_addr) = match Preprocess::check_metadata(metadata) {
        Ok((val, maybe_data_base_addr)) => (
            val,
            match maybe_data_base_addr {
                Some(v) => v,
                None => 0,
            },
        ),
        Err(e) => {
            eprintln!("{}", e);
            exit(-1)
        }
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
    let (code, data) = match CodeGen::compile(code_base_addr, instr_v) {
        Ok((c, v)) => (c, v),
        Err(e) => {
            eprintln!("{}", e);
            exit(-1)
        }
    };

    CodeGen::pack_and_write(args.output, code_base_addr, code, data_base_addr, data);
}
