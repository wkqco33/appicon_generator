use appicon_generator::cli::CliHandler;
use appicon_generator::models::Config;
use appicon_generator::services::{
    IconGenerator, ImageProcessor, ImageService, StandardIconGenerator,
};
use std::path::Path;
use tempfile::TempDir;

fn create_real_test_image(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    use image::{ImageBuffer, Rgb};

    // 100x100 흰색 이미지 생성
    let img = ImageBuffer::from_fn(100, 100, |_, _| Rgb([255u8, 255u8, 255u8]));
    img.save(path)?;
    Ok(())
}

#[test]
fn test_full_icon_generation_workflow() {
    // 임시 디렉토리 생성
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let input_file = temp_dir.path().join("test_icon.png");
    let output_dir = temp_dir.path().join("output");

    // 테스트용 이미지 생성
    create_real_test_image(&input_file).expect("Failed to create test image");

    // Config 생성
    let config = Config::new(
        input_file.to_string_lossy().to_string(),
        output_dir.to_string_lossy().to_string(),
    );

    // 설정 검증
    assert!(config.validate().is_ok());

    // 이미지 프로세서 및 아이콘 생성기 생성
    let image_service = ImageService::new();
    let icon_generator = StandardIconGenerator::new(image_service);

    // 아이콘 생성 실행
    let results = icon_generator
        .generate_all_icons(&input_file, &output_dir)
        .expect("Icon generation should succeed");

    // 결과 검증
    assert_eq!(results.len(), 2); // Android + iOS

    for result in &results {
        assert!(
            result.success,
            "Platform {:?} should succeed",
            result.platform
        );
        assert!(
            result.icons_created > 0,
            "Should create some icons for {:?}",
            result.platform
        );
    }

    // 파일 구조 검증
    assert!(
        output_dir.join("android").exists(),
        "Android directory should exist"
    );
    assert!(
        output_dir.join("ios").exists(),
        "iOS directory should exist"
    );

    // Android 아이콘 파일들 검증
    let android_folders = [
        "drawable-mdpi",
        "drawable-hdpi",
        "drawable-xhdpi",
        "drawable-xxhdpi",
        "drawable-xxxhdpi",
    ];
    for folder in &android_folders {
        let folder_path = output_dir.join("android").join(folder);
        assert!(
            folder_path.exists(),
            "Android folder {} should exist",
            folder
        );

        let icon_file = folder_path.join("ic_launcher.png");
        assert!(
            icon_file.exists(),
            "Android icon should exist in {}",
            folder
        );
    }

    // iOS 아이콘 파일들 검증 (일부만)
    let ios_icons = [
        "Icon-App-20x20@1x.png",
        "Icon-App-60x60@3x.png",
        "Icon-App-1024x1024@1x.png",
    ];
    for icon in &ios_icons {
        let icon_file = output_dir.join("ios").join(icon);
        assert!(icon_file.exists(), "iOS icon {} should exist", icon);
    }
}

#[test]
fn test_invalid_input_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let nonexistent_file = temp_dir.path().join("nonexistent.png");

    let image_service = ImageService::new();

    // 존재하지 않는 파일에 대한 검증
    assert!(!image_service.validate_image_file(&nonexistent_file));
}

#[test]
fn test_image_validation() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let image_service = ImageService::new();

    // 유효한 이미지 파일
    let valid_file = temp_dir.path().join("valid.png");
    create_real_test_image(&valid_file).expect("Failed to create test image");
    assert!(image_service.validate_image_file(&valid_file));

    // 유효하지 않은 확장자
    let invalid_file = temp_dir.path().join("invalid.txt");
    std::fs::write(&invalid_file, b"not an image").expect("Failed to create test file");
    assert!(!image_service.validate_image_file(&invalid_file));

    let corrupt_file = temp_dir.path().join("corrupt.png");
    std::fs::write(&corrupt_file, b"not an image").expect("Failed to create corrupt image");
    assert!(!image_service.validate_image_file(&corrupt_file));
}

#[test]
fn test_config_validation_integration() {
    // 유효한 설정
    let valid_config = Config::new("input.png".to_string(), "./output".to_string());
    assert!(valid_config.validate().is_ok());

    // 빈 입력 경로
    let invalid_input_config = Config::new("".to_string(), "./output".to_string());
    assert!(invalid_input_config.validate().is_err());

    // 빈 출력 경로
    let invalid_output_config = Config::new("input.png".to_string(), "".to_string());
    assert!(invalid_output_config.validate().is_err());
}

#[test]
fn test_cli_handler_integration() {
    let cli_handler = CliHandler::new();

    // CLI 핸들러가 올바르게 생성되는지 확인
    let debug_str = format!("{:?}", cli_handler);
    assert!(debug_str.contains("CliHandler"));
}

#[test]
fn test_icon_sizes_constants() {
    use appicon_generator::models::{ANDROID_SIZES, IOS_SIZES};

    // Android 아이콘 크기 검증
    assert_eq!(ANDROID_SIZES.len(), 5);
    assert_eq!(ANDROID_SIZES[0].size, 48); // mdpi
    assert_eq!(ANDROID_SIZES[4].size, 192); // xxxhdpi

    // iOS 아이콘 크기 검증
    assert_eq!(IOS_SIZES.len(), 15);
    assert_eq!(IOS_SIZES[0].size, 20); // 가장 작은 크기
    assert_eq!(IOS_SIZES[14].size, 1024); // App Store 크기
}

#[test]
fn test_generation_with_invalid_input_returns_platform_results() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let input_file = temp_dir.path().join("missing.png");
    let output_dir = temp_dir.path().join("output");
    let generator = StandardIconGenerator::new(ImageService::new());

    let results = generator
        .generate_all_icons(&input_file, &output_dir)
        .expect("Generation should return platform results");

    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|result| !result.success));
    assert!(results.iter().all(|result| result.icons_created == 0));
    assert!(results.iter().all(|result| result.error_message.is_some()));
}
