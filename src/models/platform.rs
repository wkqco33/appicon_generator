/// 플랫폼 열거형
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Android,
    Ios,
    All,
}

impl Platform {
    /// 플랫폼 이름을 문자열로 반환
    pub fn as_str(&self) -> &'static str {
        match self {
            Platform::Android => "Android",
            Platform::Ios => "iOS",
            Platform::All => "All Platforms",
        }
    }
}

/// 아이콘 생성 결과
#[derive(Debug)]
pub struct IconGenerationResult {
    pub platform: Platform,
    pub icons_created: usize,
    pub success: bool,
    pub error_message: Option<String>,
}

impl IconGenerationResult {
    /// 성공 결과 생성
    pub fn success(platform: Platform, icons_created: usize) -> Self {
        Self {
            platform,
            icons_created,
            success: true,
            error_message: None,
        }
    }

    /// 실패 결과 생성
    pub fn error(platform: Platform, error: String) -> Self {
        Self {
            platform,
            icons_created: 0,
            success: false,
            error_message: Some(error),
        }
    }

    /// 결과 요약 출력
    pub fn summary(&self) -> String {
        if self.success {
            format!(
                "✅ {}: {}개 아이콘 생성 성공",
                self.platform.as_str(),
                self.icons_created
            )
        } else {
            format!(
                "❌ {}: 생성 실패 - {}",
                self.platform.as_str(),
                self.error_message
                    .as_ref()
                    .unwrap_or(&"알 수 없는 오류".to_string())
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_as_str() {
        assert_eq!(Platform::Android.as_str(), "Android");
        assert_eq!(Platform::Ios.as_str(), "iOS");
        assert_eq!(Platform::All.as_str(), "All Platforms");
    }

    #[test]
    fn test_platform_equality() {
        assert_eq!(Platform::Android, Platform::Android);
        assert_ne!(Platform::Android, Platform::Ios);
        assert_eq!(Platform::All, Platform::All);
    }

    #[test]
    fn test_platform_clone() {
        let platform = Platform::Android;
        let cloned = platform.clone();
        assert_eq!(platform, cloned);
    }

    #[test]
    fn test_icon_generation_result_success() {
        let result = IconGenerationResult::success(Platform::Android, 5);

        assert_eq!(result.platform, Platform::Android);
        assert_eq!(result.icons_created, 5);
        assert!(result.success);
        assert!(result.error_message.is_none());
    }

    #[test]
    fn test_icon_generation_result_error() {
        let error_msg = "Test error".to_string();
        let result = IconGenerationResult::error(Platform::Ios, error_msg.clone());

        assert_eq!(result.platform, Platform::Ios);
        assert_eq!(result.icons_created, 0);
        assert!(!result.success);
        assert_eq!(result.error_message, Some(error_msg));
    }

    #[test]
    fn test_icon_generation_result_summary_success() {
        let result = IconGenerationResult::success(Platform::Android, 5);
        let summary = result.summary();

        assert!(summary.contains("✅"));
        assert!(summary.contains("Android"));
        assert!(summary.contains("5개"));
        assert!(summary.contains("성공"));
    }

    #[test]
    fn test_icon_generation_result_summary_error() {
        let result = IconGenerationResult::error(Platform::Ios, "Test error".to_string());
        let summary = result.summary();

        assert!(summary.contains("❌"));
        assert!(summary.contains("iOS"));
        assert!(summary.contains("실패"));
        assert!(summary.contains("Test error"));
    }

    #[test]
    fn test_icon_generation_result_summary_unknown_error() {
        let mut result = IconGenerationResult::error(Platform::Android, "Test".to_string());
        result.error_message = None;
        let summary = result.summary();

        assert!(summary.contains("알 수 없는 오류"));
    }
}
