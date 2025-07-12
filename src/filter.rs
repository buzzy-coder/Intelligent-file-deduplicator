use regex::Regex;
use std::fs;
use std::path::PathBuf;

pub struct FilterConfig {
    pub min_size: u64,
    pub max_size: u64,
    pub regex: Option<Regex>,
    pub allowed_extensions: Option<Vec<String>>,
}

pub fn filter_files(files: &[PathBuf], config: &FilterConfig) -> Vec<PathBuf> {
    files.iter()
        .filter_map(|file| {
            if let Ok(meta) = fs::metadata(file) {
                let size = meta.len();
                if size < config.min_size || size > config.max_size {
                    return None;
                }

                if let Some(re) = &config.regex {
                    if !re.is_match(&file.to_string_lossy()) {
                        return None;
                    }
                }

                if let Some(exts) = &config.allowed_extensions {
                    if let Some(ext) = file.extension().and_then(|e| e.to_str()) {
                        if !exts.iter().any(|e| e == ext) {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }

                Some(file.clone())
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)] 
mod tests
{
    use super::*;
    use std::fs::File;
    use std::io::Write;

    fn create_temp_file(name: &str, content: &str) -> PathBuf {
        let path = PathBuf::from(name);
        let mut file = File::create(&path).unwrap();
        write!(file, "{}", content).unwrap();
        path
    }

    #[test]
fn test_filter_by_size_and_extension() {
    let dir = std::env::temp_dir().join("filter_test_dir");
    let _ = fs::create_dir_all(&dir);

    let small_txt = dir.join("small.txt");
    let _ = File::create(&small_txt).and_then(|mut f| f.write_all(b"abc"));

    let large_txt = dir.join("large.txt");
    let _ = File::create(&large_txt).and_then(|mut f| f.write_all(&vec![0; 1_000_000]));

    let config = FilterConfig {
        min_size: 10,
        max_size: 2_000_000, // 2 MB
        regex: None,
        allowed_extensions: Some(vec!["txt".to_string()]),
    };

    let all_files = vec![small_txt.clone(), large_txt.clone()];
    let filtered = filter_files(&all_files, &config);

    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0], large_txt);

    let _ = fs::remove_file(small_txt);
    let _ = fs::remove_file(large_txt);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_filter_by_regex() {
    let dir = std::env::temp_dir().join("filter_regex_test_dir");
    let _ = fs::create_dir_all(&dir);

    let file1 = dir.join("match_abc.txt");
    let file2 = dir.join("other_xyz.txt");

    let _ = File::create(&file1).and_then(|mut f| f.write_all(b"abc"));
    let _ = File::create(&file2).and_then(|mut f| f.write_all(b"xyz"));

    let config = FilterConfig {
        min_size: 0,
        max_size: 100,
        regex: Some(Regex::new("abc").unwrap()),
        allowed_extensions: None,
    };

    let files = vec![file1.clone(), file2.clone()];
    let filtered = filter_files(&files, &config);

    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0], file1);
}

}