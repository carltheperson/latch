use elf::construct_elf;
use latch::ObjectParsingResult;
use linking::link;
use object::parse_object;
use std::env;
use std::fs;
use std::fs::write;
use std::path::PathBuf;

mod elf;
mod linking;
mod object;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut output_file = "a.out".to_string();
    let mut file_paths: Vec<PathBuf> = Vec::new();
    let mut i = 1;

    while i < args.len() {
        if args[i] == "-o" {
            if i + 1 < args.len() {
                output_file = args[i + 1].clone();
                i += 2;
            } else {
                eprintln!("Error: Missing output file after -o.");
                return;
            }
        } else {
            file_paths.push(PathBuf::from(&args[i]));
            i += 1;
        }
    }

    if file_paths.is_empty() {
        eprintln!("Error: No input files specified.");
        return;
    }

    let objects: Vec<ObjectParsingResult> = file_paths
        .iter()
        .filter_map(process_single_object_file)
        .collect();
    let elf = construct_elf(link(objects));
    write(output_file.as_str(), &elf).expect("Failed to write ELF file");
}

fn process_single_object_file(path: &PathBuf) -> Option<ObjectParsingResult> {
    match fs::read(path) {
        Ok(contents) => Some(parse_object(contents).unwrap()),
        Err(err) => {
            eprintln!("Failed to read {:?}: {}", path, err);
            None
        }
    }
}
