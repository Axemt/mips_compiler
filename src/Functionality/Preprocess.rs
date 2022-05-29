use std::ascii::AsciiExt;

use crate::Structures::{
    ArgumentBundle,
    Instruction::{Instruction, InstructionType},
};

#[derive(Debug)]
pub enum LineTag {
    Processed(String),
    Label(String),
    Ignore
}

pub fn replacement(text: String) -> Vec<(String,LineTag)> {
    //Pattern is not compatible with Strings, so no 'batch' replace :(
    text.to_ascii_lowercase()
        .split("\n")
        .map(|mut el| {

            let mut pl = el.trim()
            .replace("  ", ",") //avoid empty tokens
            .replace(", ", ",")
            .replace(" ,", ",")
            .replace("$0", "0") // convert register names
            .replace("$zero", "0")
            .replace("$at", "1")
            .replace("$v0", "2")
            .replace("$v1", "3")
            .replace("$a0", "4")
            .replace("$a1", "5")
            .replace("$a2", "6")
            .replace("$a3", "7")
            .replace("$t0", "8")
            .replace("$t1", "9")
            .replace("$t2", "10")
            .replace("$t3", "11")
            .replace("$t4", "12")
            .replace("$t5", "13")
            .replace("$t6", "14")
            .replace("$t7", "15")
            .replace("$s0", "16")
            .replace("$s1", "17")
            .replace("$s2", "18")
            .replace("$s3", "19")
            .replace("$s4", "20")
            .replace("$s5", "21")
            .replace("$s6", "22")
            .replace("$s7", "23")
            .replace("$t8", "24")
            .replace("$t9", "25")
            .replace("$k0", "26")
            .replace("$k1", "27")
            .replace("$gp", "28")
            .replace("$sp", "29")
            .replace("$s8", "30")
            .replace("$fp", "30")
            .replace("$ra", "31")
            .to_string();

            match pl.find("#") {
                Some(idx) => pl = pl.split_at(idx).0.to_string(),
                None => {}
            }

            if pl.len() == 0 { return (el.to_string(),LineTag::Ignore) }

            let ty: LineTag;
            match pl.find(":") {
                Some(idx) => { pl = pl.split_at(idx).0.to_string(); return (el.to_string(), LineTag::Label(pl.clone()) ) }
                None => {}
            }

            if let Some(idx_l) = pl.find("(") {
                if let Some(idx_r) = pl.find(")") {
                    //	<op> $0, i($0) -> <op> $0,$0,i
                    let midpart = &pl[idx_l+1..idx_r].to_string();
                    pl = pl[..idx_l].to_string();
                    
                    if let Some(idx_comma) = pl.find(",") {
                        let mut tmp = pl[..idx_comma].to_string();
                        tmp.push_str(",");
                        tmp.push_str(midpart);
                        tmp.push_str(&pl[idx_comma..]);
                        pl = tmp;
                    } else { panic!("Fatal error: replace failed; Could not find separator <,>") } 
                    dbg!(&pl);
                } else { panic!("Unmatched brace") } 
            }

            (el.to_string(), LineTag::Processed(pl))
        })
        .collect()
}
