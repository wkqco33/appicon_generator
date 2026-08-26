use image::{DynamicImage, imageops::FilterType};
use std::path::Path;

pub trait ImageProcessor {
    fn resize_image(
        &self,
        input_path: &Path,
        size: u32,
    ) -> Result<DynamicImage, Box<dyn std::error::Error>>;

    fn resize_images(
        &self,
        input_path: &Path,
        sizes: &[u32],
    ) -> Result<Vec<DynamicImage>, Box<dyn std::error::Error>> {
        sizes
            .iter()
            .map(|size| self.resize_image(input_path, *size))
            .collect()
    }

    fn validate_image_file(&self, path: &Path) -> bool;

    fn supported_formats(&self) -> Vec<&'static str>;
}

#[derive(Debug)]
pub struct ImageService;

impl ImageProcessor for ImageService {
    fn resize_image(
        &self,
        input_path: &Path,
        size: u32,
    ) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        let img = image::open(input_path)?;
        let resized = img.resize(size, size, FilterType::Lanczos3);
        Ok(resized)
    }

    fn resize_images(
        &self,
        input_path: &Path,
        sizes: &[u32],
    ) -> Result<Vec<DynamicImage>, Box<dyn std::error::Error>> {
        let image = image::open(input_path)?;
        Ok(sizes
            .iter()
            .map(|size| image.resize(*size, *size, FilterType::Lanczos3))
            .collect())
    }

    fn validate_image_file(&self, path: &Path) -> bool {
        path.is_file()
            && path
                .extension()
                .map(|extension| {
                    matches!(
                        extension.to_string_lossy().to_ascii_lowercase().as_str(),
                        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "tiff" | "webp"
                    )
                })
                .unwrap_or(false)
            && image::open(path).is_ok()
    }

    fn supported_formats(&self) -> Vec<&'static str> {
        vec!["png", "jpg", "jpeg", "gif", "bmp", "tiff", "webp"]
    }
}

impl ImageService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ImageService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn create_test_image() -> (TempDir, PathBuf) {
        let temp_dir = TempDir::new().unwrap();
        let image_path = temp_dir.path().join("test_image.png");

        let img = image::ImageBuffer::from_fn(1, 1, |_, _| image::Rgb([255u8, 255u8, 255u8]));
        img.save(&image_path).unwrap();

        (temp_dir, image_path)
    }

    #[test]
    fn test_image_service_creation() {
        let service = ImageService::new();
        let _debug = format!("{:?}", service);
    }

    #[test]
    fn test_supported_formats() {
        let service = ImageService::new();
        let formats = service.supported_formats();

        assert_eq!(formats.len(), 7);
        assert!(formats.contains(&"png"));
        assert!(formats.contains(&"jpg"));
        assert!(formats.contains(&"jpeg"));
        assert!(formats.contains(&"gif"));
        assert!(formats.contains(&"bmp"));
        assert!(formats.contains(&"tiff"));
        assert!(formats.contains(&"webp"));
    }

    #[test]
    fn test_validate_image_file_with_valid_extensions() {
        let service = ImageService::new();
        let temp_dir = TempDir::new().unwrap();

        let valid_extensions = ["png", "jpg", "jpeg", "gif", "bmp", "tiff", "webp"];

        for ext in &valid_extensions {
            let file_path = temp_dir.path().join(format!("test.{}", ext));
            create_test_image_at(&file_path);

            assert!(
                service.validate_image_file(&file_path),
                "Should validate {} files",
                ext
            );
        }
    }

    #[test]
    fn test_validate_image_file_with_invalid_extensions() {
        let service = ImageService::new();
        let temp_dir = TempDir::new().unwrap();

        let invalid_extensions = ["txt", "doc", "pdf", "mp4"];

        for ext in &invalid_extensions {
            let file_path = temp_dir.path().join(format!("test.{}", ext));
            fs::write(&file_path, b"not an image").unwrap();

            assert!(
                !service.validate_image_file(&file_path),
                "Should not validate {} files",
                ext
            );
        }
    }

    #[test]
    fn test_validate_image_file_nonexistent() {
        let service = ImageService::new();
        let nonexistent_path = Path::new("/nonexistent/file.png");

        assert!(!service.validate_image_file(nonexistent_path));
    }

    #[test]
    fn test_validate_image_file_no_extension() {
        let service = ImageService::new();
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test_file_no_ext");
        fs::write(&file_path, b"dummy content").unwrap();

        assert!(!service.validate_image_file(&file_path));
    }

    #[test]
    fn test_validate_image_file_rejects_corrupt_image_with_supported_extension() {
        let service = ImageService::new();
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("corrupt.png");
        fs::write(&file_path, b"not an image").unwrap();

        assert!(!service.validate_image_file(&file_path));
    }

    #[test]
    fn test_resize_image() {
        let service = ImageService::new();
        let (_temp_dir, image_path) = create_test_image();

        let result = service.resize_image(&image_path, 64);
        assert!(result.is_ok());

        let resized_image = result.unwrap();
        assert_eq!(resized_image.width(), 64);
        assert_eq!(resized_image.height(), 64);
    }

    #[test]
    fn test_resize_image_nonexistent_file() {
        let service = ImageService::new();
        let nonexistent_path = Path::new("/nonexistent/file.png");

        let result = service.resize_image(nonexistent_path, 64);
        assert!(result.is_err());
    }

    #[test]
    fn test_resize_image_different_sizes() {
        let service = ImageService::new();
        let (_temp_dir, image_path) = create_test_image();

        let sizes = [16, 32, 48, 64, 128, 256, 512];

        for size in &sizes {
            let result = service.resize_image(&image_path, *size);
            assert!(result.is_ok(), "Failed to resize to {}x{}", size, size);

            let resized_image = result.unwrap();
            assert_eq!(resized_image.width(), *size);
            assert_eq!(resized_image.height(), *size);
        }
    }

    #[test]
    fn test_resize_images_loads_multiple_sizes() {
        let service = ImageService::new();
        let (_temp_dir, image_path) = create_test_image();

        let images = service.resize_images(&image_path, &[16, 32, 64]).unwrap();

        assert_eq!(images.len(), 3);
        assert_eq!((images[0].width(), images[0].height()), (16, 16));
        assert_eq!((images[1].width(), images[1].height()), (32, 32));
        assert_eq!((images[2].width(), images[2].height()), (64, 64));
    }

    #[test]
    fn test_validate_image_file_case_insensitive() {
        let service = ImageService::new();
        let temp_dir = TempDir::new().unwrap();

        let mixed_case_extensions = ["PNG", "Jpg", "JPEG", "Gif"];

        for ext in &mixed_case_extensions {
            let file_path = temp_dir.path().join(format!("test.{}", ext));
            create_test_image_at(&file_path);

            assert!(
                service.validate_image_file(&file_path),
                "Should validate {} files (case insensitive)",
                ext
            );
        }
    }

    fn create_test_image_at(path: &Path) {
        let image = image::ImageBuffer::from_fn(1, 1, |_, _| image::Rgb([255u8, 255u8, 255u8]));
        image.save(path).unwrap();
    }
}
