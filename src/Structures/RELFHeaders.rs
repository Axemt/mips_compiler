#[derive(Debug)]
pub struct RelfHeader32 {

    pub e_ident_MAG: u32,
    pub e_ident_CLASS: u8,
    pub e_ident_DATA: u8,
    pub e_ident_VERSION: u8,
    pub e_ident_OSABI: u8,
    pub e_ident_ABIVERSION: u8,
    #[allow(dead_code)]
    pub e_ident_EIPAD : std::vec::Vec<u8>, //7B :(  not used, so this dirty hack with vec works
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u32,
    pub e_phoff: u32,
    pub e_shoff: u32,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16
    
}


impl Default for RelfHeader32 {
    fn default() -> Self {
        RelfHeader32 {
            e_ident_MAG: 0x7f454c46, 
            e_ident_CLASS: 0x01, 
            e_ident_DATA: 0x02, 
            e_ident_VERSION: 0x1, 
            e_ident_OSABI: 0, 
            e_ident_ABIVERSION: 0, 
            e_ident_EIPAD: vec![0; 7], 
            e_type: 0x02, 
            e_machine: 0x08,
            e_version: 0x1, 
            e_entry: 0,
            e_phoff: 0x0034,
            e_shoff: 0,
            e_flags: 0,
            e_ehsize: 0x34,
            e_phentsize: 0x20, 
            e_phnum: 0x02,
            e_shentsize: 0, 
            e_shnum: 0,
            e_shstrndx: 0
        }
        
    }
}

impl RelfHeader32{
    fn from_tuple(tuple: (u32,u8,u8,u8,u8,u8,std::vec::Vec<u8>,u16,u16,u32,u32,u32,u32,u32,u16,u16,u16,u16,u16,u16)) -> RelfHeader32 {
        RelfHeader32{
            e_ident_MAG: tuple.0, 
            e_ident_CLASS: tuple.1, 
            e_ident_DATA: tuple.2, 
            e_ident_VERSION: tuple.3, 
            e_ident_OSABI: tuple.4, 
            e_ident_ABIVERSION: tuple.5, 
            e_ident_EIPAD: tuple.6, 
            e_type: tuple.7, 
            e_machine: tuple.8,
            e_version: tuple.9, 
            e_entry: tuple.10,
            e_phoff: tuple.11,
            e_shoff: tuple.12,
            e_flags: tuple.13,
            e_ehsize: tuple.14,
            e_phentsize: tuple.15, 
            e_phnum: tuple.16,
            e_shentsize: tuple.17, 
            e_shnum: tuple.18,
            e_shstrndx: tuple.19
        }
     
     }
}

impl From<(u32,u8,u8,u8,u8,u8,std::vec::Vec<u8>,u16,u16,u32,u32,u32,u32,u32,u16,u16,u16,u16,u16,u16)> for RelfHeader32 {
    fn from(tpl: (u32,u8,u8,u8,u8,u8,std::vec::Vec<u8>,u16,u16,u32,u32,u32,u32,u32,u16,u16,u16,u16,u16,u16)) -> Self {
        RelfHeader32::from_tuple(tpl)
    }
}

impl Into<(u32,u8,u8,u8,u8,u8,std::vec::Vec<u8>,u16,u16,u32,u32,u32,u32,u32,u16,u16,u16,u16,u16,u16)> for RelfHeader32 {
    fn into(self) -> (u32,u8,u8,u8,u8,u8,std::vec::Vec<u8>,u16,u16,u32,u32,u32,u32,u32,u16,u16,u16,u16,u16,u16) {
        (
            self.e_ident_MAG,
            self.e_ident_CLASS, 
            self.e_ident_DATA, 
            self.e_ident_VERSION, 
            self.e_ident_OSABI, 
            self.e_ident_ABIVERSION, 
            self.e_ident_EIPAD, 
            self.e_type, 
            self.e_machine,
            self.e_version, 
            self.e_entry,
            self.e_phoff,
            self.e_shoff,
            self.e_flags,
            self.e_ehsize,
            self.e_phentsize, 
            self.e_phnum,
            self.e_shentsize, 
            self.e_shnum,
            self.e_shstrndx
        )
    }
}

#[derive(Debug)]
pub struct SectionHeader32 {

    pub p_type: u32,
    pub p_offset: u32,
    pub p_vaddr: u32,
    pub p_paddr: u32,
    pub p_filesz: u32,
    pub p_memsz: u32,
    pub p_flags: u32,
    #[allow(dead_code)]
    pub p_align: u32 // unused

}

impl Default for SectionHeader32 {
    fn default() -> Self {
        SectionHeader32 {
            p_type: 0, 
            p_offset: 0, 
            p_vaddr: 0,
            p_paddr: 0,
            p_filesz: 0, 
            p_memsz: 0,
            p_flags: 0,
            p_align: 0
        }
    }
}

impl SectionHeader32 {
    fn from_tuple(tuple: (u32,u32,u32,u32,u32,u32,u32,u32)) -> SectionHeader32{
        SectionHeader32 {
            p_type: tuple.0, 
            p_offset: tuple.1, 
            p_vaddr: tuple.2,
            p_paddr: tuple.3,
            p_filesz: tuple.4, 
            p_memsz: tuple.5,
            p_flags: tuple.6,
            p_align: tuple.7
        }
    }
}

impl From<(u32,u32,u32,u32,u32,u32,u32,u32)> for SectionHeader32 {
    fn from(tpl: (u32,u32,u32,u32,u32,u32,u32,u32)) -> Self {
        SectionHeader32::from_tuple(tpl)
    }
}

impl Into<(u32,u32,u32,u32,u32,u32,u32,u32)> for SectionHeader32 {
    fn into(self) -> (u32,u32,u32,u32,u32,u32,u32,u32) {
        (
            self.p_type, 
            self.p_offset, 
            self.p_vaddr,
            self.p_paddr,
            self.p_filesz, 
            self.p_memsz,
            self.p_flags,
            self.p_align
        )
    }
}
