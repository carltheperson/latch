mod object;
use object::parse_object;
use rayon::prelude::*;
use std::env;
use std::fs;
use std::path::PathBuf;

// Define the structure to hold processed data
struct SomeBytes {
    data: Vec<u8>,
}

struct ParsedObject {
    text_contents: Vec<u8>,
    data_contents: Vec<u8>,
    exported_symbols: Vec<(u64, String)>,
    imported_symbols: Vec<(u64, String)>,
}

fn main() {
    // Get file paths from command-line arguments
    let file_paths: Vec<PathBuf> = env::args().skip(1).map(PathBuf::from).collect();

    // Process files in parallel and collect results
    let some_bytes_collection: Vec<bool> = process_files_in_parallel(&file_paths);

    // Combine all SomeBytes structs here
    // let combined_data: Vec<u8> = some_bytes_collection
    //     .iter()
    //     .flat_map(|some_bytes| some_bytes.data.clone())
    //     .collect();

    // println!("Combined data length: {}", combined_data.len());
}

fn process_files_in_parallel(file_paths: &[PathBuf]) -> Vec<bool> {
    file_paths
        .iter()
        .filter_map(|path| process_single_file(path))
        .collect()
}

fn process_single_file(path: &PathBuf) -> Option<bool> {
    match fs::read(path) {
        Ok(contents) => {
            parse_object(contents);
            Some(true)
        }
        Err(err) => {
            eprintln!("Failed to read {:?}: {}", path, err);
            None
        }
    }
}
