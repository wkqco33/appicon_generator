/// iOS 아이콘 크기 정보
#[derive(Debug, Clone)]
pub struct IOSIconSize {
    pub name: &'static str,
    pub size: u32,
    pub description: &'static str,
}

impl IOSIconSize {
    pub fn new(name: &'static str, size: u32, description: &'static str) -> Self {
        Self {
            name,
            size,
            description,
        }
    }
}

/// iOS 플랫폼 아이콘 크기 정의
pub const IOS_SIZES: &[IOSIconSize] = &[
    IOSIconSize {
        name: "Icon-App-20x20@1x.png",
        size: 20,
        description: "iPhone Notification iOS 7-14",
    },
    IOSIconSize {
        name: "Icon-App-20x20@2x.png",
        size: 40,
        description: "iPhone Notification iOS 7-14",
    },
    IOSIconSize {
        name: "Icon-App-20x20@3x.png",
        size: 60,
        description: "iPhone Notification iOS 7-14",
    },
    IOSIconSize {
        name: "Icon-App-29x29@1x.png",
        size: 29,
        description: "iPhone Spotlight iOS 5,6 Settings iOS 5-14",
    },
    IOSIconSize {
        name: "Icon-App-29x29@2x.png",
        size: 58,
        description: "iPhone Spotlight iOS 5,6 Settings iOS 5-14",
    },
    IOSIconSize {
        name: "Icon-App-29x29@3x.png",
        size: 87,
        description: "iPhone Spotlight iOS 5,6 Settings iOS 5-14",
    },
    IOSIconSize {
        name: "Icon-App-40x40@1x.png",
        size: 40,
        description: "iPad Spotlight iOS 7-14",
    },
    IOSIconSize {
        name: "Icon-App-40x40@2x.png",
        size: 80,
        description: "iPhone Spotlight iOS 7-14",
    },
    IOSIconSize {
        name: "Icon-App-40x40@3x.png",
        size: 120,
        description: "iPhone Spotlight iOS 7-14",
    },
    IOSIconSize {
        name: "Icon-App-60x60@2x.png",
        size: 120,
        description: "iPhone App iOS 7-14",
    },
    IOSIconSize {
        name: "Icon-App-60x60@3x.png",
        size: 180,
        description: "iPhone App iOS 7-14",
    },
    IOSIconSize {
        name: "Icon-App-76x76@1x.png",
        size: 76,
        description: "iPad App iOS 7-14",
    },
    IOSIconSize {
        name: "Icon-App-76x76@2x.png",
        size: 152,
        description: "iPad App iOS 7-14",
    },
    IOSIconSize {
        name: "Icon-App-83.5x83.5@2x.png",
        size: 167,
        description: "iPad Pro App iOS 9-14",
    },
    IOSIconSize {
        name: "Icon-App-1024x1024@1x.png",
        size: 1024,
        description: "App Store iOS",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ios_icon_size_creation() {
        let icon = IOSIconSize::new("test_icon.png", 120, "Test description");
        assert_eq!(icon.name, "test_icon.png");
        assert_eq!(icon.size, 120);
        assert_eq!(icon.description, "Test description");
    }

    #[test]
    fn test_ios_sizes_array() {
        assert_eq!(IOS_SIZES.len(), 15);

        // 첫 번째 아이콘 (20x20@1x)
        assert_eq!(IOS_SIZES[0].name, "Icon-App-20x20@1x.png");
        assert_eq!(IOS_SIZES[0].size, 20);
        assert!(IOS_SIZES[0].description.contains("iPhone Notification"));

        // 마지막 아이콘 (1024x1024)
        let last_icon = &IOS_SIZES[IOS_SIZES.len() - 1];
        assert_eq!(last_icon.name, "Icon-App-1024x1024@1x.png");
        assert_eq!(last_icon.size, 1024);
        assert_eq!(last_icon.description, "App Store iOS");
    }

    #[test]
    fn test_ios_icon_naming_convention() {
        // 모든 iOS 아이콘이 올바른 네이밍 컨벤션을 따르는지 확인
        for icon in IOS_SIZES {
            assert!(icon.name.starts_with("Icon-App-"));
            assert!(icon.name.ends_with(".png"));
            assert!(icon.name.contains("@") || icon.name.contains("x"));
        }
    }

    #[test]
    fn test_ios_largest_icon() {
        let max_size = IOS_SIZES.iter().map(|icon| icon.size).max().unwrap();
        assert_eq!(max_size, 1024);
    }

    #[test]
    fn test_ios_smallest_icon() {
        let min_size = IOS_SIZES.iter().map(|icon| icon.size).min().unwrap();
        assert_eq!(min_size, 20);
    }

    #[test]
    fn test_ios_icon_size_debug() {
        let icon = IOSIconSize::new("test.png", 60, "Test");
        let debug_str = format!("{:?}", icon);
        assert!(debug_str.contains("IOSIconSize"));
        assert!(debug_str.contains("test.png"));
        assert!(debug_str.contains("60"));
    }

    #[test]
    fn test_ios_icon_size_clone() {
        let icon1 = IOSIconSize::new("test.png", 60, "Test");
        let icon2 = icon1.clone();

        assert_eq!(icon1.name, icon2.name);
        assert_eq!(icon1.size, icon2.size);
        assert_eq!(icon1.description, icon2.description);
    }

    #[test]
    fn test_ios_app_store_icon_exists() {
        let app_store_icon = IOS_SIZES
            .iter()
            .find(|icon| icon.description.contains("App Store"));
        assert!(app_store_icon.is_some());
        assert_eq!(app_store_icon.unwrap().size, 1024);
    }
}
