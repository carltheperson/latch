use std::collections::HashMap;

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
