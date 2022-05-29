use std::{collections::HashMap, borrow::BorrowMut, sync::Mutex};
use mut_static::MutStatic;

#[derive(Debug, Clone)]
pub enum Tag {
    Imm(u32),
    BuildPending(String),
    Resolved(u32)
}

lazy_static!(
    pub static ref TAGDICT: MutStatic<HashMap<String, Tag>> = {
        MutStatic::new()
    };
);


pub fn log_or_resolve(tag: &str) -> Tag {

    if TAGDICT.read().unwrap().contains_key(tag) {
        let entry = &TAGDICT.read().unwrap()[tag];
        if let Tag::Resolved(v) = entry {
            Tag::Resolved(*v)
        } else if let Tag::BuildPending(t) = entry {
            Tag::BuildPending(t.clone())
        } else {
            panic!()
        }
    } else {
        TAGDICT.write().unwrap().insert(tag.to_string(), Tag::BuildPending(tag.to_string()));
        Tag::BuildPending(tag.to_string())
    }
}

pub fn log_addr(tag: String, addr: u32) {
    TAGDICT.write().unwrap().insert(tag, Tag::Resolved(addr));
}

pub fn resolve(tag: String) -> u32 {
    if let Tag::Resolved(addr) = TAGDICT.read().unwrap()[&tag] {
        addr
    } else {
        panic!("Unresolved tag in compile step")
    }
}

