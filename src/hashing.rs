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
