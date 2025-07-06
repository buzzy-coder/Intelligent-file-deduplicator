// src/main.rs
mod hashing;
mod scanner;
mod filter;
mod report;
mod content_similarity;
mod image_similarity;

use hashing::HashAlgorithm;
use scanner::{scan_directory, process_files_parallel};
use filter::{filter_files, FilterConfig};
use report::{generate_report, DuplicateGroup};
use content_similarity::compare_text_files;
use image_similarity::compare_images;

#[warn(unused_imports)]
use regex::Regex;
use std::env;
use std::fs;
use std::path::PathBuf;




/// Compare files within a duplicate group using content or image similarity
fn analyze_similar_content(group: &[PathBuf]) {
    for i in 0..group.len() {
        for j in i + 1..group.len() {
            if let (Some(p1), Some(p2)) = (group.get(i), group.get(j)) {
                if p1.extension().and_then(|e| e.to_str()) == Some("txt") {
                    if let Some(score) = compare_text_files(p1, p2) {
                        println!("Text similarity between {:?} and {:?}: {}", p1, p2, score);
                    }
                }

                if ["jpg", "jpeg", "png"]
                    .contains(&p1.extension().and_then(|e| e.to_str()).unwrap_or(""))
                {
                    if let Some(score) =
                        compare_images(&p1.to_string_lossy(), &p2.to_string_lossy())
                    {
                        println!("Image similarity between {:?} and {:?}: {}", p1, p2, score);
                    }
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <folder_path>", args[0]);
        return;
    }

    let folder_path = PathBuf::from(&args[1]);

    let files = scan_directory(&folder_path);

    let config = FilterConfig {
        min_size: 1,
        max_size: 10_000_000,
        regex: Some(Regex::new(r".*").unwrap()),
        allowed_extensions: Some(vec![
            "txt".into(),
            "rs".into(),
            "jpg".into(),
            "png".into(),
        ]),
    };
    let filtered_files = filter_files(&files, &config);

    let duplicates = process_files_parallel(&filtered_files, HashAlgorithm::Sha256);

    let mut groups = Vec::new();
    if let Some(duplicates) = duplicates {
        for (hash, group) in duplicates.iter().filter(|(_, g)| g.len() > 1) {
            println!("\nDuplicate group for hash {}:", hash);
            for file in group.iter() {
                println!("  - {:?}", file);
            }

            analyze_similar_content(group);

            let total_size: u64 = group
                .iter()
                .filter_map(|f| fs::metadata(f).ok().map(|m| m.len()))
                .sum();

            groups.push(DuplicateGroup {
                hash: hash.clone(),
                files: group.clone(),
                total_size,
            });
        }
    }

    generate_report(&groups, "duplicates_report.json");
    println!("\n✅ Report saved to: duplicates_report.json");
}
