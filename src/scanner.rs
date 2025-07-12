use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

use crate::hashing::{hash_file, HashAlgorithm};

// Recursively scans the directory and collects all file paths
pub fn scan_directory(path: &PathBuf) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(scan_directory(&path));
            } else {
                files.push(path);
            }
        }
    }
    files
}

pub fn process_files_parallel(
    files: &[PathBuf],
    algo: HashAlgorithm,
) -> Option<HashMap<String, Vec<PathBuf>>> {
    let map = files
        .par_iter()
        .filter_map(|file| hash_file(file, algo.clone()).map(|hash| (hash, file.clone())))
        .fold(HashMap::new, |mut acc, (hash, file)| {
            acc.entry(hash).or_insert_with(Vec::new).push(file);
            acc
        })
        .reduce(HashMap::new, |mut acc, other| {
            for (hash, files) in other {
                acc.entry(hash).or_insert_with(Vec::new).extend(files);
            }
            acc
        });

    Some(map)
}


#[cfg(test)] 
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
fn test_scan_directory_recursively() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create nested directories and files
    let subdir = root.join("subdir");
    fs::create_dir(&subdir).unwrap();

    let file1 = root.join("file1.txt");
    let file2 = subdir.join("file2.txt");

    File::create(&file1).unwrap();
    File::create(&file2).unwrap();

    let files = scan_directory(&root.to_path_buf());

    // Assert both files are found
    assert!(files.contains(&file1));
    assert!(files.contains(&file2));
    assert_eq!(files.len(), 2);
}

}
