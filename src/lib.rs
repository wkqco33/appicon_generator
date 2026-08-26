//! AppIcon Generator
//!
//! Android와 iOS용 앱 아이콘을 생성하는 라이브러리입니다.

pub mod cli;
pub mod models;
pub mod services;

// 편의를 위한 re-export
pub use cli::{CliHandler, CliInterface};
pub use models::{
    ANDROID_SIZES, AndroidIconSize, Config, IOS_SIZES, IOSIconSize, IconGenerationResult, Platforms,
};
pub use services::{IconGenerator, ImageProcessor, ImageService, StandardIconGenerator};
