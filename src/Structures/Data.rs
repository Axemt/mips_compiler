use super::Errors::SyntaxError;
use crate::{CodeGen, TagResolution};

#[derive(Debug, PartialEq, Eq)]
pub enum DType {
    Word,
    Half,
    Byte,
    String,
    ZTerminatedString,
    Space,
}

#[derive(Debug)]
pub struct Data {
    pub contents: Vec<u8>,
    pub dt: DType,
    pub tagname: String,
}

impl From<String> for Data {
    fn from(st: String) -> Self {
        let mut s: &str = &st;
        // Format: "<tag>: .<data_type> <contents>"
        // This string must be manually split since <contents> could be a string containing any characters we set as delimiters

        let tagname = if let Some(tag_idx) = s.find(':') {
            let res = s[..tag_idx].trim();
            s = &s[tag_idx + 1..].trim();
            res.to_string()
        } else {
            panic!("Syntax Error")
        };

        let dt_pre = if let Some(dt_end_idx) = s.find(' ') {
            let res = s[1..dt_end_idx].trim();
            s = &s[dt_end_idx..].trim();
            res
        } else {
            panic!("Syntax Error")
        };

        let content_pre = s.trim();

        let (dt, contents) = match dt_pre {
            "word" => {
                let mut content: Vec<u8> = Vec::new();
                for element in content_pre.to_string().replace(" ", "").split(",") {
                    content.append(&mut to_size_N::<4>(parse_int_hex_or_dec(element)));
                }
                (DType::Word, content)
            }
            "half" => {
                let mut content: Vec<u8> = Vec::new();
                for element in content_pre.to_string().replace(" ", "").split(",") {
                    content.append(&mut to_size_N::<2>(parse_int_hex_or_dec(element)));
                }
                (DType::Half, content)
            }
            "byte" => {
                let mut content: Vec<u8> = Vec::new();
                for element in content_pre.to_string().replace(" ", "").split(",") {
                    content.append(&mut to_size_N::<1>(parse_int_hex_or_dec(element)));
                }
                (DType::Byte, content)
            }
            "asciiz" => {
                let st = match delimit_str(content_pre) {
                    Ok(s) => s,
                    Err(eobj) => panic!("{}", eobj),
                };
                let mut zterm: Vec<u8> = st.as_bytes().to_vec();
                zterm.push('\0' as u8);

                (DType::ZTerminatedString, zterm)
            }
            "ascii" => {
                let st = match delimit_str(content_pre) {
                    Ok(s) => s,
                    Err(eobj) => panic!("{}", eobj),
                };

                (DType::String, st.as_bytes().to_vec())
            }
            "space" => (
                DType::Space,
                vec![0u8; parse_int_hex_or_dec(content_pre) as usize],
            ),
            unk => panic!("Unknown data type {}", unk),
        };

        Data {
            contents,
            dt,
            tagname,
        }
    }
}

impl From<&str> for Data {
    fn from(s: &str) -> Self {
        s.to_string().into()
    }
}

fn to_size_N<const N: u32>(n_in: u32) -> Vec<u8> {
    assert!(
        (n_in as usize) < 2_usize.pow(N * 8),
        "Given number is too large for proposed size"
    );
    let mut n = n_in;
    let mask_b = 0x000000ff;
    let mut v: Vec<u8> = Vec::new();

    for _ in 0..N {
        let e = (n & mask_b).try_into().expect(&format!(
            "Internal error: masked element in position {} of {} of did not fit in 8b",
            N, n
        )); //n_in is always 32b, keep first 8b
        v.push(e);
        n = n >> 8;
    }

    v.reverse();
    v
}

/**
 *  Converts a string representation of an integer into an actual integer
 */
fn parse_int_hex_or_dec(s: &str) -> u32 {
    if s.starts_with("0X") || s.starts_with("0x") {
        u32::from_str_radix(s.trim_start_matches("0x").trim_start_matches("0X"), 16)
            .expect("Given string \"{}\" does not represent a valid base 16 integer")
    } else {
        s.parse().expect(&format!(
            "Given string \"{}\" does not represent a valid base 10 integer",
            s
        ))
    }
}

/**
 *  Takes a string containing quotes and returns Result with quotes removed
 */
fn delimit_str(st: &str) -> Result<&str, SyntaxError> {
    if let Some(left_quote_idx) = st.find('"') {
        if let Some(right_quote_idx) = st.rfind('"') && left_quote_idx != right_quote_idx {
            Ok(&st[left_quote_idx+1..right_quote_idx-1])
        } else {
            Err(SyntaxError::NoMatchingPair('"'))
        }
    } else {
        Err(SyntaxError::NoMatchingPair('"'))
    }
}
#[test]
fn conversions() {
    let n: u32 = parse_int_hex_or_dec("0x0A090807");
    let v = to_size_N::<4>(n);
    dbg!(&v);
    assert!(v == [0x0A, 0x09, 0x08, 0x07]);
    let n2: u32 = parse_int_hex_or_dec("0xffff");
    let v2 = to_size_N::<2>(n2);
    dbg!(&v2);
    assert!(v2 == [0xff, 0xff]);
    let n3: u32 = parse_int_hex_or_dec("2");
    let v3 = to_size_N::<2>(n3);
    dbg!(&v3);
    assert!(v3 == [0x00, 0x02])
}

#[test]
fn parsing() {
    TagResolution::init();
    let d: Data = "hello: .word 0xdeadbeef".into();
    dbg!(&d);
    assert!(d.contents == [0xde, 0xad, 0xbe, 0xef]);

    let d2: Data = "some_text: .asciiz \"wow, cannot believe this string is zero terminated! unbelievable what you can do with strings\"".into();
    dbg!(&d2);
    assert!(&d2.contents[0..=2] == "wow".as_bytes());
    assert!(d2.contents[d2.contents.len() - 1] == "\0".as_bytes()[0]);

    let d3: Data = "bss: .space 10".into();
    dbg!(&d3);
    assert!(d3.contents.len() == 10);

    let d4: Data = "num_vec: .half 1,  2,  3 ,4".into();
    dbg!(&d4);
    assert!(d4.contents == [0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04]);
}
