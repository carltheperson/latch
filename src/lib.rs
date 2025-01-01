use std::collections::HashMap;

pub const START_TEXT: usize = 0x401000;

#[derive(Debug, Clone)]
pub struct ExportedSymbol {
    pub index: usize,
    pub offset: u64,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct SymbolRelocation {
    pub name: String,
    pub offset: u64,
    pub r_addend: i64,
}

#[derive(Debug, Clone)]
pub struct SectionRelocation {
    pub index: usize,
    pub offset: u64,
    pub r_addend: i64,
}

#[derive(Debug, Clone)]
pub enum Relocation {
    Section(SectionRelocation),
    Symbol(SymbolRelocation),
}

#[derive(Debug)]
pub struct ObjectParsingResult {
    pub data_sections: HashMap<usize, Vec<u8>>,
    pub exported_symbols: Vec<ExportedSymbol>,
    pub relocations: Vec<Relocation>,
    pub text_contents: Vec<u8>,
}

#[derive(Debug)]
pub struct LinkingResult {
    pub text_contents: Vec<u8>,
    pub data_contents: Vec<u8>,
    pub start_addr_from_start_of_text: usize,
    pub data_virt_addr_start: usize,
}

pub fn align_to_next_page(address: usize) -> usize {
    (address + 0x1000 - 1) & !(0x1000 - 1)
}
