use goblin::container::{Container, Ctx};
use goblin::elf::*;
use goblin::mach::constants::SECTION_TYPE;
use rayon::vec;
use scroll::Pread;
use section_header::{SHT_PROGBITS, SHT_RELA, SHT_SYMTAB};
use std::io::{Read, Seek, SeekFrom};
use sym::{STB_GLOBAL, STT_FUNC, STT_NOTYPE, STT_OBJECT, STT_SECTION};

const R_X86_64_PC32: u32 = 2;

#[derive(Debug)]
struct ExportedSymbol {
    index: usize,
    offset: u64,
    name: String,
}

#[derive(Debug)]
struct SymbolRelocation {
    name: String,
    offset: u64,
    r_addend: i64,
}

#[derive(Debug)]
struct SectionRelocation {
    index: usize,
    offset: u64,
    r_addend: i64,
}

#[derive(Debug)]
enum Relocation {
    Section(SectionRelocation),
    Symbol(SymbolRelocation),
}

const ELF64_HDR_SIZE: usize = 64;

pub fn parse_object(contents: Vec<u8>) -> Result<(), &'static str> {
    let header = Elf::parse_header(&contents).map_err(|_| "parse elf header error")?;
    // dbg!(header.e_shoff);
    let ctx = Ctx {
        le: scroll::Endian::Little,
        container: Container::Big,
    };
    let section_headers = SectionHeader::parse(
        &contents,
        header.e_shoff as usize,
        header.e_shnum as usize,
        ctx,
    )
    .unwrap();

    let sh_str_idx = header.e_shstrndx as usize;
    let sh_str_offset = section_headers[sh_str_idx].sh_offset as usize;
    let sh_str_size = section_headers[sh_str_idx].sh_size as usize;
    let str_table = &contents[sh_str_offset..sh_str_offset + sh_str_size];

    // Iterate over section headers and print their names
    for header in &section_headers {
        let name_offset = header.sh_name as usize;
        let name = str_table.pread::<&str>(name_offset).unwrap();
        // println!("{}", name);
    }

    let symtab_header = section_headers
        .iter()
        .find(|header| {
            let name_offset = header.sh_name as usize;
            let name = str_table.pread::<&str>(name_offset).unwrap_or("");
            name == ".symtab"
        })
        .ok_or("No .symtab section found")?;
    let symtab_offset = symtab_header.sh_offset as usize;
    let symtab_size = symtab_header.sh_size as usize;
    let entry_size = symtab_header.sh_entsize as usize;

    let mut symbols = Vec::new();
    for i in (0..symtab_size).step_by(entry_size) {
        let ctx = Ctx {
            le: scroll::Endian::Little,
            container: Container::Big,
        };
        let sym: Sym = contents.pread_with(symtab_offset + i, ctx).unwrap();
        symbols.push(sym);
    }

    // Locate the string table for symbols
    let strtab_header = &section_headers[symtab_header.sh_link as usize];
    let strtab_offset = strtab_header.sh_offset as usize;
    let strtab_size = strtab_header.sh_size as usize;
    let strtab = &contents[strtab_offset..strtab_offset + strtab_size];

    let mut exported_symbols: Vec<ExportedSymbol> = vec![];
    let mut section_indexes_needed: Vec<usize> = vec![];

    for (i, symbol) in symbols.iter().enumerate() {
        let st_type = symbol.st_type();
        if symbol.st_name != 0
            && symbol.st_bind() == STB_GLOBAL
            && (st_type == STT_FUNC || st_type == STT_OBJECT)
        {
            let name_offset = symbol.st_name as usize;
            let name = strtab.pread::<&str>(name_offset).unwrap();
            exported_symbols.push(ExportedSymbol {
                index: i,
                name: name.to_string(),
                offset: symbol.st_value,
            });
        } else if st_type == STT_SECTION {
            section_indexes_needed.push(symbol.st_shndx);
        }
    }

    // A bold assumption to make, but one we're making
    let data_sections: Vec<usize> = section_indexes_needed.iter().skip(1).cloned().collect();

    let mut data = std::collections::HashMap::new();
    for &section_index in &data_sections {
        if section_index < section_headers.len() {
            let section_header = &section_headers[section_index];
            let section_offset = section_header.sh_offset as usize;
            let section_size = section_header.sh_size as usize;

            if section_offset + section_size <= contents.len() {
                let section_data = &contents[section_offset..section_offset + section_size];
                data.insert(section_index, section_data.to_vec());
            }
        }
    }

    let rela_text_header = section_headers
        .iter()
        .find(|header| {
            let name_offset = header.sh_name as usize;
            let name = str_table.pread::<&str>(name_offset).unwrap_or("");
            name == ".rela.text"
        })
        .ok_or("No .rela.text section found")?;

    // Parse `.rela.text` entries
    let rela_text_offset = rela_text_header.sh_offset as usize;
    let rela_text_size = rela_text_header.sh_size as usize;
    let rela_entry_size = rela_text_header.sh_entsize as usize;
    let mut relocations: Vec<Relocation> = vec![];
    for i in (0..rela_text_size).step_by(rela_entry_size) {
        let ctx = Ctx {
            le: scroll::Endian::Little,
            container: Container::Big,
        };
        let rela: Reloc = contents
            .pread_with(rela_text_offset + i, (true, ctx))
            .unwrap();
        let symbol = symbols[rela.r_sym];
        if rela.r_type == R_X86_64_PC32 {
            relocations.push(Relocation::Section(SectionRelocation {
                index: symbol.st_shndx,
                offset: rela.r_offset,
                r_addend: rela.r_addend.unwrap(),
            }));
        } else {
            let name_offset = symbol.st_name as usize;
            let name = strtab.pread::<&str>(name_offset).unwrap();
            relocations.push(Relocation::Symbol(SymbolRelocation {
                name: name.to_string(),
                offset: rela.r_offset,
                r_addend: rela.r_addend.unwrap(),
            }));
        }
    }

    dbg!(relocations);

    Ok(())
}
