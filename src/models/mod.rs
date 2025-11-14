/// Android 플랫폼 관련 모델
pub mod android;

/// iOS 플랫폼 관련 모델
pub mod ios;

/// 플랫폼 공통 모델
pub mod platform;

/// 설정 관련 모델
pub mod config;

// 편의를 위한 re-export
pub use android::{AndroidIconSize, ANDROID_SIZES};
pub use ios::{IOSIconSize, IOS_SIZES};
pub use platform::{Platform, IconGenerationResult};
pub use config::Config;