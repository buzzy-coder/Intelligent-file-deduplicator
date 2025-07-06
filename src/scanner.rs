use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::hashing::{hash_file, HashAlgorithm};

pub fn scan_directory(path: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(scan_directory(&path));// scanner.rs
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;
use crate::hashing::{hash_file, HashAlgorithm};
use std::collections::HashMap;

/// Recursively scans the directory and collects all file paths
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
#[warn(dead_code)]
// Hashes files in parallel and returns a map of hash to list of paths
pub fn process_files_parallel(files: &[PathBuf], algo: HashAlgorithm) -> HashMap<String, Vec<PathBuf>> {
    files.par_iter()
        .filter_map(|file| {
            let hash = hash_file(file, algo.clone())?;
            Some((hash, file.clone()))
        })
        .fold(
            || HashMap::<String, Vec<PathBuf>>::new(),
            |mut acc, (hash, path)| {
                acc.entry(hash).or_default().push(path);
                acc
            }
        )
        .reduce(
            || HashMap::<String, Vec<PathBuf>>::new(),
            |mut acc, map| {
                for (hash, paths) in map {
                    acc.entry(hash).or_default().extend(paths);
                }
                acc
            }
        )
}

            } else {
                files.push(path);
            }
        }
    }
    files
}

pub fn process_files_parallel(files: &[PathBuf], algo: HashAlgorithm) -> Option<HashMap<String, Vec<PathBuf>>> {

    let results = Arc::new(Mutex::new(HashMap::new()));

    files.par_iter().for_each(|file| {
        if let Some(hash) = hash_file(file, algo.clone()) {
            let mut map = results.lock().unwrap();
            map.entry(hash).or_insert_with(Vec::new).push(file.clone());
        }
    });

    // Display duplicates
    let map = Arc::try_unwrap(results).unwrap().into_inner().unwrap();
    for (hash, group) in map.iter().filter(|(_, g)| g.len() > 1) {
        println!("Duplicate Hash: {}", hash);
        for file in group {
            println!("  - {:?}", file);
        }
    }
    Some(map)
}
