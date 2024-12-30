use goblin::container::{Container, Ctx};
use goblin::elf::*;
use header::*;
use latch::LinkingResult;
use scroll::{Pread, Pwrite, LE};
use section_header::{SHF_ALLOC, SHF_EXECINSTR, SHF_WRITE, SHT_PROGBITS, SHT_STRTAB};
use sym::{STB_GLOBAL, STT_FUNC, STT_OBJECT};

// #[derive(Debug)]
// pub struct LinkingResult {
//     pub text_contents: Vec<u8>,
//     pub data_contents: Vec<u8>,
//     pub start_addr_from_start_of_text: usize,
//     pub data_virt_addr_start: usize,
// }

const START_TEXT: usize = 0x401000;

pub fn construct_elf(info: LinkingResult) -> Vec<u8> {
    let header = Header {
        e_ident: [
            0x7f,
            b'E',
            b'L',
            b'F',
            ELFCLASS64,
            ELFDATA2LSB,
            EV_CURRENT,
            ELFOSABI_SYSV,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        e_type: ET_EXEC,
        e_machine: EM_X86_64,
        e_version: EV_CURRENT as u32,
        e_entry: (START_TEXT + info.start_addr_from_start_of_text) as u64,
        e_phoff: 0,
        e_shoff: 0,
        e_flags: 0,
        e_ehsize: Header::size(Ctx {
            le: scroll::Endian::Little,
            container: Container::Big,
        }) as u16,
        e_phentsize: 0,
        e_phnum: 0,
        e_shentsize: SectionHeader::size(Ctx {
            le: scroll::Endian::Little,
            container: Container::Big,
        }) as u16,
        e_shnum: 3,    // .text, .data, .shstrtab
        e_shstrndx: 2, // Index of .shstrtab
    };

    let mut elf = Elf::lazy_parse(header).unwrap();

    let text_offset = 0x1000; // File offset for .text
    let data_offset = text_offset + info.text_contents.len();

    // Section header strings
    let mut shstrtab = b"\0.text\0.data\0.shstrtab\0".to_vec();

    // .text section
    let text_section = SectionHeader {
        sh_name: 1, // Offset in shstrtab
        sh_type: SHT_PROGBITS,
        sh_flags: (SHF_ALLOC | SHF_EXECINSTR) as u64,
        sh_addr: START_TEXT as u64,
        sh_offset: text_offset as u64,
        sh_size: info.text_contents.len() as u64,
        sh_addralign: 16,
        ..SectionHeader::default()
    };

    // .data section
    let data_section = SectionHeader {
        sh_name: 7, // Offset in shstrtab
        sh_type: SHT_PROGBITS,
        sh_flags: (SHF_ALLOC | SHF_WRITE) as u64,
        sh_addr: info.data_virt_addr_start as u64,
        sh_offset: data_offset as u64,
        sh_size: info.data_contents.len() as u64,
        sh_addralign: 16,
        ..SectionHeader::default()
    };

    // .shstrtab section
    let shstrtab_section = SectionHeader {
        sh_name: 13, // Offset in shstrtab
        sh_type: SHT_STRTAB,
        sh_offset: (data_offset + info.data_contents.len()) as u64,
        sh_size: shstrtab.len() as u64,
        sh_addralign: 1,
        ..SectionHeader::default()
    };

    let header_size = SectionHeader::size(Ctx {
        le: scroll::Endian::Little,
        container: Container::Big,
    });

    elf.section_headers.push(text_section);
    elf.section_headers.push(data_section);
    elf.section_headers.push(shstrtab_section.clone());

    let sh_offset = shstrtab_section.sh_offset + shstrtab_section.sh_size;
    elf.header.e_shoff = sh_offset; // Section headers offset
    elf.header.e_shentsize = header_size as u16;
    elf.header.e_shnum = elf.section_headers.len() as u16;
    elf.header.e_shstrndx = 2; // Index of .shstrtab

    // Prepare the binary
    let mut binary = vec![0u8; sh_offset as usize + elf.section_headers.len() * header_size];

    // Write .text section
    binary[text_offset..text_offset + info.text_contents.len()]
        .copy_from_slice(&info.text_contents);

    // Write .data section
    binary[data_offset..data_offset + info.data_contents.len()]
        .copy_from_slice(&info.data_contents);

    // Write .shstrtab section
    binary[(data_offset + info.data_contents.len())
        ..(data_offset + info.data_contents.len() + shstrtab.len())]
        .copy_from_slice(&shstrtab);

    // Write section headers
    let mut offset = sh_offset as usize;
    for section in &elf.section_headers {
        binary
            .pwrite_with(
                section.clone(),
                offset,
                Ctx {
                    le: scroll::Endian::Little,
                    container: Container::Big,
                },
            )
            .unwrap();
        offset += header_size;
    }

    // Write ELF header
    binary.pwrite_with(elf.header, 0, LE).unwrap();

    binary
}
