use std::collections::HashMap;

use latch::{align_to_next_page, LinkingResult, ObjectParsingResult, Relocation};

const START_TEXT: usize = 0x401000;

#[derive(Debug)]
struct SectionInfo {
    text_i: usize,
    data_is: HashMap<usize, usize>,
}

pub fn link(objects: Vec<ObjectParsingResult>) -> LinkingResult {
    let mut total_text = vec![];
    let mut total_data = vec![];

    let mut object_section_infos: Vec<SectionInfo> = vec![];
    let mut symbol_names_to_full_offset: HashMap<String, usize> = HashMap::new();
    let mut relocations: Vec<Vec<Relocation>> = vec![];

    for object in objects {
        let mut section_info = SectionInfo {
            text_i: total_text.len(),
            data_is: HashMap::new(),
        };
        for (i, data) in object.data_sections {
            section_info.data_is.insert(i, total_data.len());
            total_data.extend(data);
        }

        object_section_infos.push(section_info);

        for symbol in object.exported_symbols {
            symbol_names_to_full_offset
                .insert(symbol.name, total_text.len() + symbol.offset as usize);
        }

        relocations.push(object.relocations);

        total_text.extend(object.text_contents);
    }

    let data_start = align_to_next_page(START_TEXT + total_text.len());

    for (i, relocations) in relocations.into_iter().enumerate() {
        let info = &object_section_infos[i];
        for rela in relocations {
            match rela {
                Relocation::Section(rel_inf) => {
                    let section_addr_no_page_info = info.data_is.get(&rel_inf.index).unwrap();
                    let section_addr =
                        data_start + section_addr_no_page_info + rel_inf.offset as usize;
                    let path_apply_addr = START_TEXT + rel_inf.offset as usize;
                    let diff = section_addr - path_apply_addr;
                    let location_to_patch = info.text_i + rel_inf.offset as usize;

                    total_text[location_to_patch..location_to_patch + 4]
                        .copy_from_slice(&(diff as u32).to_le_bytes());
                }
                Relocation::Symbol(rel_inf) => {
                    let symbol_addr = symbol_names_to_full_offset.get(&rel_inf.name).unwrap();
                    let location_to_patch = info.text_i + rel_inf.offset as usize;
                    let diff = *symbol_addr as isize - location_to_patch as isize
                        + rel_inf.r_addend as isize;

                    total_text[location_to_patch..location_to_patch + 4]
                        .copy_from_slice(&(diff as u32).to_le_bytes());
                }
            }
        }
    }

    let start_addr = *symbol_names_to_full_offset.get("_start").unwrap();

    LinkingResult {
        data_contents: total_data,
        text_contents: total_text,
        data_virt_addr_start: data_start,
        start_addr_from_start_of_text: start_addr,
    }
}
