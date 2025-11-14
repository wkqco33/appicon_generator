/// 이미지 처리 관련 서비스
pub mod image_service;

/// 아이콘 생성 관련 서비스
pub mod icon_generator;

// 편의를 위한 re-export
pub use image_service::{ImageProcessor, ImageService};
pub use icon_generator::{IconGenerator, StandardIconGenerator};