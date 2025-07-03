mod hashing;
mod scanner;

use std::env;
use std::path::PathBuf;
use hashing::HashAlgorithm;
use scanner::{scan_directory, process_files_parallel};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <folder_path>", args[0]);
        return;
    }

    let folder_path = PathBuf::from(&args[1]);
    let files = scan_directory(&folder_path);
    process_files_parallel(&files, HashAlgorithm::Sha256);
}

