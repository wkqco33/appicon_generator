use image::{DynamicImage, imageops::FilterType};
use std::path::Path;

/// 이미지 처리 트레잇
pub trait ImageProcessor {
    /// 이미지를 지정된 크기로 리사이징
    fn resize_image(
        &self,
        input_path: &Path,
        size: u32,
    ) -> Result<DynamicImage, Box<dyn std::error::Error>>;

    /// 이미지 파일 존재 여부 및 형식 확인
    fn validate_image_file(&self, path: &Path) -> bool;

    /// 지원하는 이미지 형식 목록 반환
    fn supported_formats(&self) -> Vec<&'static str>;
}

/// 이미지 처리 관련 서비스
#[derive(Debug)]
pub struct ImageService;

impl ImageProcessor for ImageService {
    /// 이미지를 지정된 크기로 리사이징
    ///
    /// # Arguments
    /// * `input_path` - 입력 이미지 파일 경로
    /// * `size` - 출력 이미지 크기 (정사각형)
    ///
    /// # Returns
    /// 리사이징된 이미지 또는 에러
    fn resize_image(
        &self,
        input_path: &Path,
        size: u32,
    ) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        let img = image::open(input_path)?;
        let resized = img.resize(size, size, FilterType::Lanczos3);
        Ok(resized)
    }

    /// 이미지 파일 존재 여부 확인
    ///
    /// # Arguments
    /// * `path` - 확인할 파일 경로
    ///
    /// # Returns
    /// 파일 존재 여부
    fn validate_image_file(&self, path: &Path) -> bool {
        if !path.exists() {
            return false;
        }

        // 파일 확장자 확인
        if let Some(extension) = path.extension() {
            let ext = extension.to_string_lossy().to_lowercase();
            matches!(
                ext.as_str(),
                "png" | "jpg" | "jpeg" | "gif" | "bmp" | "tiff" | "webp"
            )
        } else {
            false
        }
    }

    /// 지원하는 이미지 형식 목록 반환
    fn supported_formats(&self) -> Vec<&'static str> {
        vec!["png", "jpg", "jpeg", "gif", "bmp", "tiff", "webp"]
    }
}

impl ImageService {
    /// ImageService 인스턴스 생성
    pub fn new() -> Self {
        Self
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

        // 1x1 픽셀 PNG 이미지 생성 (u8 타입 사용)
        let img = image::ImageBuffer::from_fn(1, 1, |_, _| image::Rgb([255u8, 255u8, 255u8]));
        img.save(&image_path).unwrap();

        (temp_dir, image_path)
    }

    #[test]
    fn test_image_service_creation() {
        let service = ImageService::new();
        // 단순히 생성이 되는지 확인
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
            fs::write(&file_path, b"dummy content").unwrap();

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
            fs::write(&file_path, b"dummy content").unwrap();

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
    fn test_validate_image_file_case_insensitive() {
        let service = ImageService::new();
        let temp_dir = TempDir::new().unwrap();

        // 대소문자 혼합 확장자 테스트
        let mixed_case_extensions = ["PNG", "Jpg", "JPEG", "Gif"];

        for ext in &mixed_case_extensions {
            let file_path = temp_dir.path().join(format!("test.{}", ext));
            fs::write(&file_path, b"dummy content").unwrap();

            assert!(
                service.validate_image_file(&file_path),
                "Should validate {} files (case insensitive)",
                ext
            );
        }
    }
}
