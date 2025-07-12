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


#[cfg(test)] 
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_generate_report() {
        let group1 = DuplicateGroup {
            hash: "abc123".to_string(),
            files: vec![PathBuf::from("file1.txt"), PathBuf::from("file2.txt")],
            total_size: 2048,
        };
        let group2 = DuplicateGroup {
            hash: "def456".to_string(),
            files: vec![PathBuf::from("file3.txt")],
            total_size: 1024,
        };

        let groups = vec![group1, group2];
        let output_path = "test_report.json";

        generate_report(&groups, output_path);

        assert!(fs::metadata(output_path).is_ok());
        fs::remove_file(output_path).unwrap();
    }
}
