pub mod hashing;
pub mod scanner;
pub mod filter;
pub mod report;
pub mod content_similarity;
pub mod image_similarity;
pub mod quarantine;

use hashing::HashAlgorithm;
use scanner::{scan_directory, process_files_parallel};
use filter::{filter_files, FilterConfig};
use report::{generate_report, DuplicateGroup};
use content_similarity::compare_text_files;
use image_similarity::compare_images;

use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::env;

// Compare files within a duplicate group using content or image similarity
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

pub fn run_deduplication(folder_path: &PathBuf, report_name: &str) {
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

    generate_report(&groups, report_name);
    println!("\n✅ Report saved to: {}", report_name);
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <folder_path>", args[0]);
        return;
    }

    let folder_path = PathBuf::from(&args[1]);
    run_deduplication(&folder_path, "duplicates_report.json");
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;
    use std::fs::File;
    use tempfile::tempdir;

    use crate::run_deduplication;

    #[test]
    fn test_run_deduplication_on_temp_dir() {
        let dir = tempdir().unwrap();
        let folder = dir.path();

        let file1 = folder.join("file1.txt");
        let file2 = folder.join("file2.txt");
        let file3 = folder.join("file3.txt");

        // file1 and file2 are duplicates
        File::create(&file1)
            .unwrap()
            .write_all(b"duplicate content")
            .unwrap();
        File::create(&file2)
            .unwrap()
            .write_all(b"duplicate content")
            .unwrap();
        File::create(&file3)
            .unwrap()
            .write_all(b"unique content")
            .unwrap();

        let report_path = folder.join("test_report.json");

        run_deduplication(&folder.to_path_buf(), report_path.to_str().unwrap());

        assert!(report_path.exists());

        let report_contents = fs::read_to_string(&report_path).unwrap();
        assert!(report_contents.contains("file1.txt"));
        assert!(report_contents.contains("file2.txt"));
        assert!(!report_contents.contains("file3.txt"));

        fs::remove_file(report_path).unwrap();
    }
}
