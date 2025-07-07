use std::fs::File;
use std::io::{Read, BufReader};
use std::path::Path;

use sha2::{Sha256, Digest};
use blake3;
use twox_hash::XxHash64;
use std::hash::Hasher;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum HashAlgorithm {
    Sha256,
    Blake3,
    XxHash,
}

pub fn hash_file(path: &Path, algo: HashAlgorithm) -> Option<String> {
    let file = File::open(path).ok()?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).ok()?;

    let hash = match algo {
        HashAlgorithm::Sha256 => format!("{:x}", Sha256::digest(&buffer)),
        HashAlgorithm::Blake3 => blake3::hash(&buffer).to_hex().to_string(),
        HashAlgorithm::XxHash => {
            let mut hasher = XxHash64::default();
            hasher.write(&buffer);
            format!("{:x}", hasher.finish())
        }
    };

    Some(hash)
}






#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    fn create_temp_file(name: &str, content: &str) -> PathBuf {
        let path = PathBuf::from(name);
        let mut file = File::create(&path).unwrap();
        write!(file, "{}", content).unwrap();
        path
    }

    #[test]
    fn test_hash_file_sha256() {
        let path = create_temp_file("test_sha256.txt", "hello rust");
        let result = hash_file(&path, HashAlgorithm::Sha256);
        assert!(result.is_some());
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_hash_file_blake3() {
        let path = create_temp_file("test_blake3.txt", "hello rust");
        let result = hash_file(&path, HashAlgorithm::Blake3);
        assert!(result.is_some());
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_hash_file_xxhash() {
        let path = create_temp_file("test_xxhash.txt", "hello rust");
        let result = hash_file(&path, HashAlgorithm::XxHash);
        assert!(result.is_some());
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_hash_file_nonexistent() {
        let path = PathBuf::from("non_existent_file.txt");
        let result = hash_file(&path, HashAlgorithm::Sha256);
        assert!(result.is_none());
    }
}

