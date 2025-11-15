/// Android 아이콘 크기 정보
#[derive(Debug, Clone)]
pub struct AndroidIconSize {
    pub name: &'static str,
    pub size: u32,
    pub folder: &'static str,
}

/// Android 플랫폼 아이콘 크기 정의
pub const ANDROID_SIZES: &[AndroidIconSize] = &[
    AndroidIconSize {
        name: "ic_launcher",
        size: 48,
        folder: "drawable-mdpi",
    },
    AndroidIconSize {
        name: "ic_launcher",
        size: 72,
        folder: "drawable-hdpi",
    },
    AndroidIconSize {
        name: "ic_launcher",
        size: 96,
        folder: "drawable-xhdpi",
    },
    AndroidIconSize {
        name: "ic_launcher",
        size: 144,
        folder: "drawable-xxhdpi",
    },
    AndroidIconSize {
        name: "ic_launcher",
        size: 192,
        folder: "drawable-xxxhdpi",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_android_icon_size_creation() {
        let icon = AndroidIconSize {
            name: "test_icon",
            size: 48,
            folder: "drawable-test",
        };
        assert_eq!(icon.name, "test_icon");
        assert_eq!(icon.size, 48);
        assert_eq!(icon.folder, "drawable-test");
    }

    #[test]
    fn test_android_sizes_array() {
        assert_eq!(ANDROID_SIZES.len(), 5);

        // 첫 번째 아이콘 (mdpi)
        assert_eq!(ANDROID_SIZES[0].name, "ic_launcher");
        assert_eq!(ANDROID_SIZES[0].size, 48);
        assert_eq!(ANDROID_SIZES[0].folder, "drawable-mdpi");

        // 마지막 아이콘 (xxxhdpi)
        assert_eq!(ANDROID_SIZES[4].name, "ic_launcher");
        assert_eq!(ANDROID_SIZES[4].size, 192);
        assert_eq!(ANDROID_SIZES[4].folder, "drawable-xxxhdpi");
    }

    #[test]
    fn test_android_sizes_ordering() {
        // 크기가 오름차순으로 정렬되어 있는지 확인
        for i in 0..ANDROID_SIZES.len() - 1 {
            assert!(
                ANDROID_SIZES[i].size < ANDROID_SIZES[i + 1].size,
                "Android sizes should be in ascending order"
            );
        }
    }

    #[test]
    fn test_android_icon_size_debug() {
        let icon = AndroidIconSize {
            name: "test",
            size: 48,
            folder: "drawable-test",
        };
        let debug_str = format!("{:?}", icon);
        assert!(debug_str.contains("AndroidIconSize"));
        assert!(debug_str.contains("test"));
        assert!(debug_str.contains("48"));
    }

    #[test]
    fn test_android_icon_size_clone() {
        let icon1 = AndroidIconSize {
            name: "test",
            size: 48,
            folder: "drawable-test",
        };
        let icon2 = icon1.clone();

        assert_eq!(icon1.name, icon2.name);
        assert_eq!(icon1.size, icon2.size);
        assert_eq!(icon1.folder, icon2.folder);
    }
}
