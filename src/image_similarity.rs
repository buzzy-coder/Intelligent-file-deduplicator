use img_hash::{HasherConfig, HashAlg};
use img_hash::image::open;

/// Compare two images using perceptual hashing (returns Hamming distance)
pub fn compare_images(img1_path: &str, img2_path: &str) -> Option<u32> {
    let img1 = open(img1_path).ok()?;
    let img2 = open(img2_path).ok()?;

    let hasher = HasherConfig::new()
        .hash_alg(HashAlg::Mean)
        .to_hasher();

    let hash1 = hasher.hash_image(&img1);
    let hash2 = hasher.hash_image(&img2);

    Some(hash1.dist(&hash2))
}
