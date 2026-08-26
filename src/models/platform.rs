/// 플랫폼 열거형
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platforms {
    Android,
    Ios,
}

/// 아이콘 생성 결과
#[derive(Debug)]
pub struct IconGenerationResult {
    pub platform: Platforms,
    pub icons_created: usize,
    pub success: bool,
    pub error_message: Option<String>,
}

impl IconGenerationResult {
    /// 성공 결과 생성
    pub fn success(platform: Platforms, icons_created: usize) -> Self {
        Self {
            platform,
            icons_created,
            success: true,
            error_message: None,
        }
    }

    /// 실패 결과 생성
    pub fn error(platform: Platforms, error: String) -> Self {
        Self {
            platform,
            icons_created: 0,
            success: false,
            error_message: Some(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_equality() {
        assert_eq!(Platforms::Android, Platforms::Android);
        assert_ne!(Platforms::Android, Platforms::Ios);
    }

    #[test]
    fn test_platform_clone() {
        let platform = Platforms::Android;
        let cloned = platform;
        assert_eq!(platform, cloned);
    }

    #[test]
    fn test_icon_generation_result_success() {
        let result = IconGenerationResult::success(Platforms::Android, 5);

        assert_eq!(result.platform, Platforms::Android);
        assert_eq!(result.icons_created, 5);
        assert!(result.success);
        assert!(result.error_message.is_none());
    }

    #[test]
    fn test_icon_generation_result_error() {
        let error_msg = "Test error".to_string();
        let result = IconGenerationResult::error(Platforms::Ios, error_msg.clone());

        assert_eq!(result.platform, Platforms::Ios);
        assert_eq!(result.icons_created, 0);
        assert!(!result.success);
        assert_eq!(result.error_message, Some(error_msg));
    }
}
