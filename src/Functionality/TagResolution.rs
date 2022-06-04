use mut_static::MutStatic;
use std::{borrow::BorrowMut, collections::HashMap, sync::Mutex};

#[derive(Debug, Clone)]
pub enum Tag {
    Imm(u32),
    BuildPending(String),
    Resolved(u32),
}

lazy_static! {
    pub static ref TAGDICT: MutStatic<HashMap<String, Tag>> = { MutStatic::new() };
}

pub fn init() {
    TAGDICT.set(std::collections::HashMap::new()).unwrap();
    TAGDICT
        .write()
        .unwrap()
        .insert(".text".into(), Tag::BuildPending(".text".into()));
    TAGDICT
        .write()
        .unwrap()
        .insert(".data".into(), Tag::BuildPending(".data".into()));
}

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
        TAGDICT
            .write()
            .unwrap()
            .insert(tag.to_string(), Tag::BuildPending(tag.to_string()));
        Tag::BuildPending(tag.to_string())
    }
}

pub fn log_addr(tag: String, addr: u32) {
    dbg!(&tag);
    TAGDICT.write().unwrap().insert(tag, Tag::Resolved(addr));
}

pub fn resolve(tag: String) -> u32 {
    if let Tag::Resolved(addr) = TAGDICT.read().unwrap()[&tag] {
        addr
    } else {
        panic!("Unresolved tag in compile step")
    }
}
