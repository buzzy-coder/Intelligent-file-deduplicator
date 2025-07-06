use std::fs;
use std::path::Path;
use strsim::levenshtein;

/// Compare two text files using Levenshtein edit distance
pub fn compare_text_files(path1: &Path, path2: &Path) -> Option<usize> {
    let text1 = fs::read_to_string(path1).ok()?;
    let text2 = fs::read_to_string(path2).ok()?;
    Some(levenshtein(&text1, &text2))
}