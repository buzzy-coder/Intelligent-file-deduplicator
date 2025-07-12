use std::fs;
use std::path::Path;
use strsim::levenshtein;

/// Compare two text files using Levenshtein edit distance
pub fn compare_text_files(path1: &Path, path2: &Path) -> Option<usize> {
    let text1 = fs::read_to_string(path1).ok()?;
    let text2 = fs::read_to_string(path2).ok()?;
    Some(levenshtein(&text1, &text2))
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    fn create_temp_file(name: &str, content: &str) -> std::path::PathBuf {
        let path = Path::new(name);
        let mut file = File::create(&path).unwrap();
        write!(file, "{}", content).unwrap();
        path.to_path_buf()
    }

    #[test]
    fn test_compare_text_files() {
        let path1 = Path::new("test1.txt");
        let path2 = Path::new("test2.txt");

        let _ = File::create(&path1).and_then(|mut f| write!(f, "hello world"));
        let _ = File::create(&path2).and_then(|mut f| write!(f, "hello world"));

        let distance = compare_text_files(&path1, &path2).unwrap();
        assert_eq!(distance, 0);

        let _ = std::fs::remove_file(path1);
        let _ = std::fs::remove_file(path2);
    }
}
