mod linking;
mod object;
use latch::ObjectParsingResult;
use linking::link;
use object::parse_object;
use rayon::prelude::*;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let file_paths: Vec<PathBuf> = env::args().skip(1).map(PathBuf::from).collect();

    let objects: Vec<ObjectParsingResult> = process_files_in_parallel(&file_paths);
    link(objects);
}

fn process_files_in_parallel(file_paths: &[PathBuf]) -> Vec<ObjectParsingResult> {
    file_paths
        .iter()
        .filter_map(|path| process_single_file(path))
        .collect()
}

fn process_single_file(path: &PathBuf) -> Option<ObjectParsingResult> {
    match fs::read(path) {
        Ok(contents) => Some(parse_object(contents).unwrap()),
        Err(err) => {
            eprintln!("Failed to read {:?}: {}", path, err);
            None
        }
    }
}
