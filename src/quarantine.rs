use std::fs;
use std::path::{Path, PathBuf};

/// Moves a file to a quarantine directory
pub fn move_to_quarantine(file: &Path, quarantine_dir: &Path) -> std::io::Result<PathBuf> {
    fs::create_dir_all(quarantine_dir)?;
    let file_name = file.file_name().unwrap();
    let new_path = quarantine_dir.join(file_name);
    fs::rename(file, &new_path)?;
    Ok(new_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    use crate::quarantine::move_to_quarantine;

    #[test]
    fn test_move_to_quarantine_success() {
        let temp_dir = tempdir().unwrap();
        let source_dir = temp_dir.path().join("source");
        let quarantine_dir = temp_dir.path().join("quarantine");

        // Create source and quarantine directories
        fs::create_dir_all(&source_dir).unwrap();
        fs::create_dir_all(&quarantine_dir).unwrap();

        // Create a test file in the source directory
        let file_path = source_dir.join("sample.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "quarantine me").unwrap();

        // Perform the quarantine move
        let result = move_to_quarantine(&file_path, &quarantine_dir);
        assert!(result.is_ok());

        let new_path = result.unwrap();

        // Ensure the new path exists in quarantine
        assert!(new_path.exists());
        assert_eq!(new_path.parent().unwrap(), quarantine_dir);

        // Ensure the original file no longer exists
        assert!(!file_path.exists());
    }
}
