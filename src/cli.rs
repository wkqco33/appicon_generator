use crate::models::Config;
use clap::{Arg, Command};

/// CLI 인터페이스 트레잇
pub trait CliInterface {
    /// 명령줄 인수를 파싱하여 설정 반환
    fn parse_args(&self) -> Config;
}

/// CLI 인터페이스 처리
#[derive(Debug)]
pub struct CliHandler;

impl CliHandler {
    /// 새로운 CliHandler 인스턴스 생성
    pub fn new() -> Self {
        Self
    }
}

impl CliInterface for CliHandler {
    /// 명령줄 인수를 파싱하여 설정 반환
    ///
    /// # Returns
    /// 파싱된 설정 구조체
    fn parse_args(&self) -> Config {
        let matches = Command::new("AppIcon Generator")
            .version("1.0")
            .author("Your Name")
            .about("Android와 iOS용 앱 아이콘을 생성합니다")
            .arg(
                Arg::new("input")
                    .short('i')
                    .long("input")
                    .value_name("IMAGE_PATH")
                    .help("입력 이미지 파일 경로")
                    .required(true),
            )
            .arg(
                Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("OUTPUT_DIR")
                    .help("출력 디렉토리 경로 (기본값: 현재 디렉토리)")
                    .default_value("."),
            )
            .get_matches();

        let input_path = matches.get_one::<String>("input").unwrap().clone();
        let output_dir = matches.get_one::<String>("output").unwrap().clone();

        Config::new(input_path, output_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_handler_creation() {
        let handler = CliHandler::new();
        let debug_str = format!("{:?}", handler);
        assert!(debug_str.contains("CliHandler"));
    }

    #[test]
    fn test_config_creation_from_cli() {
        // 이 테스트는 실제 clap이 아닌 Config 생성 로직만 테스트
        let config = Config::new("test_input.png".to_string(), "./test_output".to_string());

        assert_eq!(config.input_path, "test_input.png");
        assert_eq!(config.output_dir, "./test_output");
    }
}

// CLI 통합 테스트 (실제 명령줄 인수 파싱 테스트)
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        // Config 검증 로직 테스트
        let valid_config = Config::new("input.png".to_string(), "./output".to_string());
        assert!(valid_config.validate().is_ok());

        let invalid_config = Config::new("".to_string(), "".to_string());
        assert!(invalid_config.validate().is_err());
    }
}
