mod cli;
mod models;
mod services;

use cli::{CliHandler, CliInterface};
use services::{IconGenerator, ImageProcessor, ImageService, StandardIconGenerator};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // CLI 핸들러 생성 및 인수 파싱
    let cli_handler = CliHandler::new();
    let config = cli_handler.parse_args();

    match config.validate() {
        Ok(_) => {
            println!("✅ 경로 - {}", config.summary());
        }
        Err(e) => {
            eprintln!("❌ 잘못된 구성입니다: {}", e);
            std::process::exit(1);
        }
    }

    let input_path = Path::new(&config.input_path);
    let output_dir = Path::new(&config.output_dir);

    // 이미지 프로세서 생성
    let image_processor = ImageService::new();

    // 입력 파일 유효성 검사
    if !image_processor.validate_image_file(input_path) {
        eprintln!(
            "❌ 입력 파일이 존재하지 않거나 지원되지 않는 형식입니다: {}",
            input_path.display()
        );
        eprintln!(
            "지원하는 형식: {}",
            image_processor.supported_formats().join(", ")
        );
        std::process::exit(1);
    }

    // 아이콘 생성기 생성
    let icon_generator = StandardIconGenerator::new(image_processor);

    // 아이콘 생성 실행
    match icon_generator.generate_all_icons(input_path, output_dir) {
        Ok(results) => {
            let total_icons: usize = results.iter().map(|r| r.icons_created).sum();
            let failed_count = results.iter().filter(|r| !r.success).count();

            if failed_count == 0 {
                println!(
                    "🎉 총 {}개의 아이콘이 성공적으로 생성되었습니다!",
                    total_icons
                );
            } else {
                println!(
                    "⚠️  일부 아이콘 생성에 실패했습니다. 성공: {}개, 실패: {}개",
                    total_icons, failed_count
                );
                for result in &results {
                    if !result.success {
                        eprintln!(
                            "  {:?}: {}",
                            result.platform,
                            result.error_message.as_ref().unwrap()
                        );
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 아이콘 생성 중 오류가 발생했습니다: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
