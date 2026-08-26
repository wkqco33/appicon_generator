use crate::models::Config;
use std::sync::{Arc, Mutex};
use wrcli::{Command, Flag, FlagValue};

pub trait CliInterface {
    fn parse_args(&self) -> Config;
}

#[derive(Debug)]
pub struct CliHandler;

impl CliHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CliHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl CliInterface for CliHandler {
    fn parse_args(&self) -> Config {
        let parsed_config = Arc::new(Mutex::new(None));
        let config_slot = Arc::clone(&parsed_config);

        let result = Command::new("appicon_generator")
            .version("1.0")
            .short("Android와 iOS용 앱 아이콘을 생성합니다")
            .flag(
                Flag::new(
                    "input",
                    FlagValue::String(String::new()),
                    "입력 이미지 파일 경로",
                )
                .short('i')
                .required(),
            )
            .flag(
                Flag::new(
                    "output",
                    FlagValue::String(".".to_string()),
                    "출력 디렉토리 경로 (기본값: 현재 디렉토리)",
                )
                .short('o'),
            )
            .on_run(move |ctx| {
                let config = Config::new(
                    ctx.flags
                        .get_string("input")
                        .unwrap_or_default()
                        .to_string(),
                    ctx.flags.get_string("output").unwrap_or(".").to_string(),
                );
                *config_slot.lock().expect("CLI config lock poisoned") = Some(config);
            })
            .execute();

        if let Err(error) = result {
            eprintln!("CLI 오류: {}", error);
            std::process::exit(1);
        }

        match parsed_config
            .lock()
            .expect("CLI config lock poisoned")
            .take()
        {
            Some(config) => config,
            // `wrcli` handles terminal commands such as --help and --version
            // without invoking the application callback.
            None => std::process::exit(0),
        }
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
        let config = Config::new("test_input.png".to_string(), "./test_output".to_string());

        assert_eq!(config.input_path, "test_input.png");
        assert_eq!(config.output_dir, "./test_output");
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let valid_config = Config::new("input.png".to_string(), "./output".to_string());
        assert!(valid_config.validate().is_ok());

        let invalid_config = Config::new("".to_string(), "".to_string());
        assert!(invalid_config.validate().is_err());
    }
}
