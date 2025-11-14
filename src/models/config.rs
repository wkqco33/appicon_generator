/// CLI 설정 구조체
#[derive(Debug, Clone)]
pub struct Config {
    pub input_path: String,
    pub output_dir: String,
}

impl Config {
    /// 새로운 Config 인스턴스 생성
    pub fn new(input_path: String, output_dir: String) -> Self {
        Self {
            input_path,
            output_dir,
        }
    }

    /// 설정 유효성 검사
    pub fn validate(&self) -> Result<(), String> {
        if self.input_path.is_empty() {
            return Err("입력 경로가 비어있습니다".to_string());
        }

        if self.output_dir.is_empty() {
            return Err("출력 경로가 비어있습니다".to_string());
        }

        Ok(())
    }

    /// 설정 요약 출력
    pub fn summary(&self) -> String {
        format!("입력: {}, 출력: {}", self.input_path, self.output_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = Config::new(
            "/path/to/input.png".to_string(),
            "/path/to/output".to_string(),
        );

        assert_eq!(config.input_path, "/path/to/input.png");
        assert_eq!(config.output_dir, "/path/to/output");
    }

    #[test]
    fn test_config_validate_success() {
        let config = Config::new("input.png".to_string(), "output".to_string());

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validate_empty_input() {
        let config = Config::new("".to_string(), "output".to_string());

        let result = config.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "입력 경로가 비어있습니다");
    }

    #[test]
    fn test_config_validate_empty_output() {
        let config = Config::new("input.png".to_string(), "".to_string());

        let result = config.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "출력 경로가 비어있습니다");
    }

    #[test]
    fn test_config_summary() {
        let config = Config::new("test.png".to_string(), "./output".to_string());

        let summary = config.summary();
        assert!(summary.contains("test.png"));
        assert!(summary.contains("./output"));
        assert!(summary.contains("입력:"));
        assert!(summary.contains("출력:"));
    }

    #[test]
    fn test_config_clone() {
        let config1 = Config::new("input.png".to_string(), "output".to_string());
        let config2 = config1.clone();

        assert_eq!(config1.input_path, config2.input_path);
        assert_eq!(config1.output_dir, config2.output_dir);
    }

    #[test]
    fn test_config_debug() {
        let config = Config::new("test.png".to_string(), "./out".to_string());
        let debug_str = format!("{:?}", config);

        assert!(debug_str.contains("Config"));
        assert!(debug_str.contains("test.png"));
        assert!(debug_str.contains("./out"));
    }
}
