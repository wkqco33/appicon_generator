use crate::models::{ANDROID_SIZES, IOS_SIZES, IconGenerationResult, Platforms};
use crate::services::image_service::ImageProcessor;
use image::ImageFormat;
use std::fs;
use std::path::Path;

pub trait IconGenerator {
    fn create_android_icons(
        &self,
        input_path: &Path,
        output_dir: &Path,
    ) -> Result<IconGenerationResult, Box<dyn std::error::Error>>;

    fn create_ios_icons(
        &self,
        input_path: &Path,
        output_dir: &Path,
    ) -> Result<IconGenerationResult, Box<dyn std::error::Error>>;

    fn generate_all_icons(
        &self,
        input_path: &Path,
        output_dir: &Path,
    ) -> Result<Vec<IconGenerationResult>, Box<dyn std::error::Error>>;
}

#[derive(Debug)]
pub struct StandardIconGenerator<T: ImageProcessor> {
    image_processor: T,
}

impl<T: ImageProcessor> StandardIconGenerator<T> {
    pub fn new(image_processor: T) -> Self {
        Self { image_processor }
    }
}

impl<T: ImageProcessor> IconGenerator for StandardIconGenerator<T> {
    fn create_android_icons(
        &self,
        input_path: &Path,
        output_dir: &Path,
    ) -> Result<IconGenerationResult, Box<dyn std::error::Error>> {
        println!("📱 Android 아이콘 생성 중...");

        let android_dir = output_dir.join("android");
        fs::create_dir_all(&android_dir)?;

        let sizes: Vec<_> = ANDROID_SIZES.iter().map(|icon| icon.size).collect();
        let images = match self.image_processor.resize_images(input_path, &sizes) {
            Ok(images) => images,
            Err(error) => {
                return Ok(IconGenerationResult::error(
                    Platforms::Android,
                    error.to_string(),
                ));
            }
        };
        for (icon_size, resized_img) in ANDROID_SIZES.iter().zip(images) {
            let folder_path = android_dir.join(icon_size.folder);
            fs::create_dir_all(&folder_path)?;
            let output_path = folder_path.join(format!("{}.png", icon_size.name));
            if let Err(error) = resized_img.save_with_format(&output_path, ImageFormat::Png) {
                return Ok(IconGenerationResult::error(
                    Platforms::Android,
                    error.to_string(),
                ));
            }
            println!(
                "  ✓ {}x{} → {}",
                icon_size.size,
                icon_size.size,
                output_path.display()
            );
        }

        Ok(IconGenerationResult::success(
            Platforms::Android,
            ANDROID_SIZES.len(),
        ))
    }

    fn create_ios_icons(
        &self,
        input_path: &Path,
        output_dir: &Path,
    ) -> Result<IconGenerationResult, Box<dyn std::error::Error>> {
        println!("🍎 iOS 아이콘 생성 중...");

        let ios_dir = output_dir.join("ios");
        fs::create_dir_all(&ios_dir)?;

        let sizes: Vec<_> = IOS_SIZES.iter().map(|icon| icon.size).collect();
        let images = match self.image_processor.resize_images(input_path, &sizes) {
            Ok(images) => images,
            Err(error) => {
                return Ok(IconGenerationResult::error(
                    Platforms::Ios,
                    error.to_string(),
                ));
            }
        };
        for (icon_size, resized_img) in IOS_SIZES.iter().zip(images) {
            let output_path = ios_dir.join(icon_size.name);
            if let Err(error) = resized_img.save_with_format(&output_path, ImageFormat::Png) {
                return Ok(IconGenerationResult::error(
                    Platforms::Ios,
                    error.to_string(),
                ));
            }
            println!(
                "  ✓ {}x{} → {} ({})",
                icon_size.size, icon_size.size, icon_size.name, icon_size.description
            );
        }

        Ok(IconGenerationResult::success(
            Platforms::Ios,
            IOS_SIZES.len(),
        ))
    }

    fn generate_all_icons(
        &self,
        input_path: &Path,
        output_dir: &Path,
    ) -> Result<Vec<IconGenerationResult>, Box<dyn std::error::Error>> {
        fs::create_dir_all(output_dir)?;

        println!("🚀 앱 아이콘 생성을 시작합니다...");
        println!("📁 입력 파일: {}", input_path.display());
        println!("📁 출력 디렉토리: {}", output_dir.display());
        println!();

        let mut results = Vec::new();

        match self.create_android_icons(input_path, output_dir) {
            Ok(result) => {
                if result.success {
                    println!("✅ Android 아이콘 {}개 생성 완료", result.icons_created);
                } else {
                    eprintln!(
                        "❌ Android 아이콘 생성 실패: {}",
                        result.error_message.as_ref().unwrap()
                    );
                }
                results.push(result);
            }
            Err(e) => {
                eprintln!("❌ Android 아이콘 생성 실패: {}", e);
                results.push(IconGenerationResult::error(
                    Platforms::Android,
                    e.to_string(),
                ));
            }
        }

        println!();

        match self.create_ios_icons(input_path, output_dir) {
            Ok(result) => {
                if result.success {
                    println!("✅ iOS 아이콘 {}개 생성 완료", result.icons_created);
                } else {
                    eprintln!(
                        "❌ iOS 아이콘 생성 실패: {}",
                        result.error_message.as_ref().unwrap()
                    );
                }
                results.push(result);
            }
            Err(e) => {
                eprintln!("❌ iOS 아이콘 생성 실패: {}", e);
                results.push(IconGenerationResult::error(Platforms::Ios, e.to_string()));
            }
        }

        println!();
        println!("✅ 앱 아이콘 생성이 완료되었습니다!");
        println!("📱 Android 아이콘: {}/android/", output_dir.display());
        println!("🍎 iOS 아이콘: {}/ios/", output_dir.display());

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::image_service::ImageProcessor;
    use image::{DynamicImage, ImageBuffer, Rgb};
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[derive(Debug)]
    struct MockImageProcessor;

    impl ImageProcessor for MockImageProcessor {
        fn resize_image(
            &self,
            _input_path: &Path,
            size: u32,
        ) -> Result<DynamicImage, Box<dyn std::error::Error>> {
            let img = ImageBuffer::from_fn(size, size, |_, _| Rgb([255u8, 255u8, 255u8]));
            Ok(DynamicImage::ImageRgb8(img))
        }

        fn validate_image_file(&self, _path: &Path) -> bool {
            true
        }

        fn supported_formats(&self) -> Vec<&'static str> {
            vec!["png", "jpg", "jpeg"]
        }
    }

    fn setup_test_environment() -> (TempDir, PathBuf, PathBuf) {
        let temp_dir = TempDir::new().unwrap();
        let input_path = temp_dir.path().join("test_input.png");
        let output_dir = temp_dir.path().join("output");

        fs::write(&input_path, b"dummy image content").unwrap();
        fs::create_dir_all(&output_dir).unwrap();

        (temp_dir, input_path, output_dir)
    }

    #[test]
    fn test_standard_icon_generator_creation() {
        let image_processor = MockImageProcessor;
        let generator = StandardIconGenerator::new(image_processor);

        let debug_str = format!("{:?}", generator);
        assert!(debug_str.contains("StandardIconGenerator"));
    }

    #[test]
    fn test_create_android_icons_success() {
        let image_processor = MockImageProcessor;
        let generator = StandardIconGenerator::new(image_processor);
        let (_temp_dir, input_path, output_dir) = setup_test_environment();

        let result = generator.create_android_icons(&input_path, &output_dir);

        assert!(result.is_ok());
        let icon_result = result.unwrap();
        assert!(icon_result.success);
        assert_eq!(icon_result.platform, Platforms::Android);
        assert_eq!(icon_result.icons_created, ANDROID_SIZES.len());

        let android_dir = output_dir.join("android");
        assert!(android_dir.exists());
    }

    #[test]
    fn test_create_ios_icons_success() {
        let image_processor = MockImageProcessor;
        let generator = StandardIconGenerator::new(image_processor);
        let (_temp_dir, input_path, output_dir) = setup_test_environment();

        let result = generator.create_ios_icons(&input_path, &output_dir);

        assert!(result.is_ok());
        let icon_result = result.unwrap();
        assert!(icon_result.success);
        assert_eq!(icon_result.platform, Platforms::Ios);
        assert_eq!(icon_result.icons_created, IOS_SIZES.len());

        // iOS 폴더가 생성되었는지 확인
        let ios_dir = output_dir.join("ios");
        assert!(ios_dir.exists());
    }

    #[test]
    fn test_generate_all_icons() {
        let image_processor = MockImageProcessor;
        let generator = StandardIconGenerator::new(image_processor);
        let (_temp_dir, input_path, output_dir) = setup_test_environment();

        let result = generator.generate_all_icons(&input_path, &output_dir);

        assert!(result.is_ok());
        let results = result.unwrap();
        assert_eq!(results.len(), 2); // Android + iOS

        // 모든 결과가 성공적인지 확인
        for result in &results {
            assert!(
                result.success,
                "Platform {:?} should succeed",
                result.platform
            );
        }

        // 양쪽 폴더가 모두 생성되었는지 확인
        assert!(output_dir.join("android").exists());
        assert!(output_dir.join("ios").exists());
    }

    #[test]
    fn test_android_folder_structure() {
        let image_processor = MockImageProcessor;
        let generator = StandardIconGenerator::new(image_processor);
        let (_temp_dir, input_path, output_dir) = setup_test_environment();

        let _ = generator.create_android_icons(&input_path, &output_dir);

        let android_dir = output_dir.join("android");
        for icon_size in ANDROID_SIZES {
            let folder_path = android_dir.join(icon_size.folder);
            assert!(
                folder_path.exists(),
                "Folder {} should exist",
                icon_size.folder
            );

            let icon_file = folder_path.join(format!("{}.png", icon_size.name));
            assert!(
                icon_file.exists(),
                "Icon file should exist at {:?}",
                icon_file
            );
        }
    }

    #[test]
    fn test_ios_icon_files_created() {
        let image_processor = MockImageProcessor;
        let generator = StandardIconGenerator::new(image_processor);
        let (_temp_dir, input_path, output_dir) = setup_test_environment();

        let _ = generator.create_ios_icons(&input_path, &output_dir);

        let ios_dir = output_dir.join("ios");
        for icon_size in IOS_SIZES {
            let icon_file = ios_dir.join(icon_size.name);
            assert!(
                icon_file.exists(),
                "iOS icon file {} should exist",
                icon_size.name
            );
        }
    }
}
