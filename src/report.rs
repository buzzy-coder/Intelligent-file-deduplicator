use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct DuplicateGroup {
    pub hash: String,
    pub files: Vec<PathBuf>,
    pub total_size: u64,
}

pub fn generate_report(groups: &[DuplicateGroup], output: &str) {
    let json = serde_json::to_string_pretty(groups).unwrap();
    let mut file = File::create(output).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
