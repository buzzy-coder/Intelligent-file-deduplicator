mod hashing;
mod scanner;

use std::path::PathBuf;
use hashing::HashAlgorithm;
use scanner::{scan_directory, process_files_parallel};

fn main() {
    let target_dir = PathBuf::from("C:/Users/honey/OneDrive/Documents/file-testing for deduplicator");
    let files = scan_directory(&target_dir);

    println!("Found {} files. Processing...", files.len());
    process_files_parallel(&files, HashAlgorithm::Sha256);
}

