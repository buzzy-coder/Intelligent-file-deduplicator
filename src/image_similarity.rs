use img_hash::{HasherConfig, HashAlg};
use img_hash::image::open;

// Compare two images using perceptual hashing (returns Hamming distance)
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

#[cfg(test)]
mod tests {
    use super::*;
    use img_hash::image::{DynamicImage, RgbImage, Rgb};

    // In-memory comparison helper for tests only
    fn compare_images_in_memory(img1: &DynamicImage, img2: &DynamicImage) -> u32 {
        let hasher = HasherConfig::new()
            .hash_alg(HashAlg::Mean)
            .to_hasher();

        let hash1 = hasher.hash_image(img1);
        let hash2 = hasher.hash_image(img2);

        hash1.dist(&hash2)
    }

    fn create_test_image(color: [u8; 3]) -> DynamicImage {
        let img = RgbImage::from_pixel(64, 64, Rgb(color));
        DynamicImage::ImageRgb8(img)
    }

    #[test]
    fn test_identical_images_in_memory() {
        let img1 = create_test_image([255, 0, 0]);
        let img2 = create_test_image([255, 0, 0]);
        let distance = compare_images_in_memory(&img1, &img2);
        assert_eq!(distance, 0);
    }

}