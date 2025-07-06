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