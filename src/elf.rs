use goblin::container::{Container, Ctx};
use goblin::elf::*;
use header::*;
use latch::{align_to_next_page, LinkingResult, START_TEXT};
use program_header::{PF_R, PF_W, PF_X, PT_GNU_STACK, PT_LOAD};
use scroll::{Pwrite, LE};
use section_header::{SHF_ALLOC, SHF_EXECINSTR, SHF_WRITE, SHT_NULL, SHT_PROGBITS, SHT_STRTAB};

pub fn construct_elf(info: LinkingResult) -> Vec<u8> {
    let ctx = Ctx {
        le: scroll::Endian::Little,
        container: Container::Big,
    };

    let shstrtab = b"\0.text\0.data\0.shstrtab\0".to_vec();

    let elf_header_size = Header::size(ctx);
    let single_program_header_size = ProgramHeader::size(ctx);
    let single_section_header_size = SectionHeader::size(ctx);

    // Program header details (segments to load into memory: interpreter, text, data, gnu stack)
    let number_of_program_headers = 4;
    let interpreter_segment_offset = 0x0;
    let interpreter_segment_size =
        elf_header_size + single_program_header_size * number_of_program_headers;
    let interpreter_segment_virt_addr = 0x400000;
    let interpreter_segment_flags = PF_R;
    let text_segment_offset = 0x1000;
    let text_segment_size = info.text_contents.len();
    let text_segment_virt_addr = START_TEXT;
    let text_segment_flags = PF_X | PF_R;
    let data_segment_offset = align_to_next_page(text_segment_offset + text_segment_size);
    let data_segment_size = info.data_contents.len();
    let data_segment_virt_addr = info.data_virt_addr_start;
    let data_segment_flags = PF_R;
    let gnu_stack_segment_offset = 0x0;
    let gnu_stack_segment_size = 0x0;
    let gnu_stack_segment_virt_addr = 0x0;
    let gnu_stack_segment_flags = PF_R | PF_W;

    let start_of_program_headers = elf_header_size;
    let program_headers = vec![
        // interpreter
        ProgramHeader {
            p_type: PT_LOAD,
            p_offset: interpreter_segment_offset as u64,
            p_vaddr: interpreter_segment_virt_addr as u64,
            p_paddr: interpreter_segment_virt_addr as u64,
            p_filesz: interpreter_segment_size as u64,
            p_memsz: interpreter_segment_size as u64,
            p_flags: interpreter_segment_flags,
            p_align: 0x1000,
        },
        // text
        ProgramHeader {
            p_type: PT_LOAD,
            p_offset: text_segment_offset as u64,
            p_vaddr: text_segment_virt_addr as u64,
            p_paddr: text_segment_virt_addr as u64,
            p_filesz: text_segment_size as u64,
            p_memsz: text_segment_size as u64,
            p_flags: text_segment_flags,
            p_align: 0x1000,
        },
        // data
        ProgramHeader {
            p_type: PT_LOAD,
            p_offset: data_segment_offset as u64,
            p_vaddr: data_segment_virt_addr as u64,
            p_paddr: data_segment_virt_addr as u64,
            p_filesz: data_segment_size as u64,
            p_memsz: data_segment_size as u64,
            p_flags: data_segment_flags,
            p_align: 0x1000,
        },
        // gnu stack
        ProgramHeader {
            p_type: PT_GNU_STACK,
            p_offset: gnu_stack_segment_offset as u64,
            p_vaddr: gnu_stack_segment_virt_addr as u64,
            p_paddr: gnu_stack_segment_virt_addr as u64,
            p_filesz: gnu_stack_segment_size as u64,
            p_memsz: gnu_stack_segment_size as u64,
            p_flags: gnu_stack_segment_flags,
            p_align: 0x10,
        },
    ];

    // Section details (sections: null, .text, .data, .shstrtab)
    let text_section_offset = 0x1000;
    let text_section_size = info.text_contents.len();
    let text_section_virt_addr = START_TEXT;
    let data_section_offset = align_to_next_page(text_section_offset + text_section_size);
    let data_section_size = info.data_contents.len();
    let data_section_virt_addr = info.data_virt_addr_start;
    let shstrtab_section_offset = data_section_offset + data_section_size;
    let shstrtab_section_size = shstrtab.len();
    let shstrtab_section_virt_addr = 0x0;

    // Section header details (sections: null, .text, .data, .shstrtab)
    let start_of_section_headers = shstrtab_section_offset + shstrtab_section_size;
    let section_headers = vec![
        // null
        SectionHeader {
            sh_name: 0,
            sh_type: SHT_NULL,
            sh_flags: 0,
            sh_addr: 0x0,
            sh_offset: 0x0,
            sh_size: 0x0,
            sh_addralign: 0,
            ..SectionHeader::default()
        },
        // .text
        SectionHeader {
            sh_name: 1,
            sh_type: SHT_PROGBITS,
            sh_flags: (SHF_ALLOC | SHF_EXECINSTR) as u64,
            sh_addr: text_section_virt_addr as u64,
            sh_offset: text_section_offset as u64,
            sh_size: text_section_size as u64,
            sh_addralign: 1,
            ..SectionHeader::default()
        },
        // .data
        SectionHeader {
            sh_name: 7,
            sh_type: SHT_PROGBITS,
            sh_flags: (SHF_ALLOC | SHF_WRITE) as u64,
            sh_addr: data_section_virt_addr as u64,
            sh_offset: data_section_offset as u64,
            sh_size: data_section_size as u64,
            sh_addralign: 1,
            ..SectionHeader::default()
        },
        // .shstrtab
        SectionHeader {
            sh_name: 13,
            sh_type: SHT_STRTAB,
            sh_offset: shstrtab_section_offset as u64,
            sh_size: shstrtab_section_size as u64,
            sh_addr: shstrtab_section_virt_addr as u64,
            sh_addralign: 1,
            ..SectionHeader::default()
        },
    ];

    let full_file_size =
        start_of_section_headers + section_headers.len() * single_section_header_size;

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
        e_entry: (text_section_virt_addr + info.start_addr_from_start_of_text) as u64,
        e_phoff: start_of_program_headers as u64,
        e_shoff: start_of_section_headers as u64,
        e_flags: 0,
        e_ehsize: elf_header_size as u16,
        e_phentsize: single_program_header_size as u16,
        e_phnum: program_headers.len() as u16,
        e_shentsize: single_section_header_size as u16,
        e_shnum: section_headers.len() as u16,
        e_shstrndx: (section_headers.len() - 1) as u16, // Assume last is .shstrtab
    };

    let mut final_blob = vec![0u8; full_file_size];

    final_blob.pwrite_with(header, 0, LE).unwrap();

    // Write program headers into the ELF
    for (i, ph) in program_headers.into_iter().enumerate() {
        let offset = start_of_program_headers + i * single_program_header_size;
        final_blob.pwrite_with(ph, offset, ctx).unwrap();
    }

    // Write section headers into the ELF
    for (i, sh) in section_headers.into_iter().enumerate() {
        let offset = start_of_section_headers + i * single_section_header_size;
        final_blob.pwrite_with(sh, offset, ctx).unwrap();
    }

    // Write .text section content
    final_blob[text_section_offset..(text_section_offset + text_section_size)]
        .copy_from_slice(&info.text_contents);

    // Write .data section content
    final_blob[data_section_offset..(data_section_offset + data_section_size)]
        .copy_from_slice(&info.data_contents);

    // Write .shstrtab section content
    final_blob[shstrtab_section_offset..(shstrtab_section_offset + shstrtab_section_size)]
        .copy_from_slice(&shstrtab);

    final_blob
}
