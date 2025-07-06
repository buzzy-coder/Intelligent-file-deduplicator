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