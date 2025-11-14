# 📱 App Icon Generator

Android와 iOS용 앱 아이콘을 자동으로 생성하는 Rust CLI 도구입니다.

## ✨ 주요 기능

- 🤖 **자동 아이콘 생성**: 하나의 원본 이미지에서 모든 필요한 크기의 아이콘을 생성
- 📱 **Android 지원**: 5가지 밀도별 아이콘 (mdpi, hdpi, xhdpi, xxhdpi, xxxhdpi)
- 🍎 **iOS 지원**: 15가지 용도별 아이콘 (알림, 설정, 스포트라이트, 앱 아이콘, 앱스토어)
- 🎨 **고품질 리샘플링**: Lanczos3 알고리즘으로 최고 품질의 이미지 변환
- 📁 **자동 폴더 구조**: 플랫폼별 표준 폴더 구조로 자동 정리
- ⚡ **빠른 처리**: Rust의 성능으로 빠른 배치 처리

## 🚀 빠른 시작

### 설치 방법

#### Cargo를 통한 설치

```bash
# 저장소 클론
git clone <repository-url>
cd appicon_generator

# 빌드 및 설치
make install
```

#### 직접 빌드

```bash
# 개발용 빌드
make build

# 릴리스 빌드
make release
```

### 사용 방법

```bash
# 기본 사용법
appicon_generator --input your_image.png --output ./icons

# 현재 디렉토리에 생성
appicon_generator --input logo.png

# 도움말 보기
appicon_generator --help
```

## 📂 생성되는 파일 구조

```text
output_folder/
├── android/
│   ├── drawable-mdpi/ic_launcher.png        # 48x48px
│   ├── drawable-hdpi/ic_launcher.png        # 72x72px
│   ├── drawable-xhdpi/ic_launcher.png       # 96x96px
│   ├── drawable-xxhdpi/ic_launcher.png      # 144x144px
│   └── drawable-xxxhdpi/ic_launcher.png     # 192x192px
└── ios/
    ├── Icon-App-20x20@1x.png               # 20x20px (알림)
    ├── Icon-App-20x20@2x.png               # 40x40px (알림 @2x)
    ├── Icon-App-20x20@3x.png               # 60x60px (알림 @3x)
    ├── Icon-App-29x29@1x.png               # 29x29px (설정)
    ├── Icon-App-29x29@2x.png               # 58x58px (설정 @2x)
    ├── Icon-App-29x29@3x.png               # 87x87px (설정 @3x)
    ├── Icon-App-40x40@1x.png               # 40x40px (스포트라이트)
    ├── Icon-App-40x40@2x.png               # 80x80px (스포트라이트 @2x)
    ├── Icon-App-40x40@3x.png               # 120x120px (스포트라이트 @3x)
    ├── Icon-App-60x60@2x.png               # 120x120px (앱 아이콘 @2x)
    ├── Icon-App-60x60@3x.png               # 180x180px (앱 아이콘 @3x)
    ├── Icon-App-76x76@1x.png               # 76x76px (iPad 앱)
    ├── Icon-App-76x76@2x.png               # 152x152px (iPad 앱 @2x)
    ├── Icon-App-83.5x83.5@2x.png           # 167x167px (iPad Pro)
    └── Icon-App-1024x1024@1x.png           # 1024x1024px (앱스토어)
```

## 🛠️ 개발

### 요구사항

- Rust 1.70+ (2024 Edition)
- Cargo

### 개발 환경 설정

```bash
# 저장소 클론
git clone <repository-url>
cd appicon_generator

# 종속성 설치 및 빌드
make deps
make build

# 테스트 실행
make test

# 개발용 실행
make dev
```

### 사용 가능한 Make 명령어

```bash
make help          # 사용 가능한 명령어 목록 보기
make build         # 개발용 빌드
make release       # 릴리스 빌드 (최적화)
make test          # 모든 테스트 실행
make test-unit     # 유닛 테스트만 실행
make test-int      # 통합 테스트만 실행
make install       # 시스템에 설치
make clean         # 빌드 아티팩트 정리
make deps          # 종속성 확인
make fmt           # 코드 포맷팅
make lint          # 코드 린팅
make example       # 예제 실행
```

### 프로젝트 구조

```text
src/
├── main.rs                 # CLI 진입점
├── lib.rs                  # 라이브러리 인터페이스
├── cli.rs                  # CLI 인터페이스
├── models/                 # 데이터 모델
│   ├── mod.rs
│   ├── android.rs          # Android 아이콘 크기 정의
│   ├── ios.rs              # iOS 아이콘 크기 정의
│   ├── platform.rs         # 플랫폼 enum 및 결과 타입
│   └── config.rs           # CLI 설정 구조체
└── services/               # 비즈니스 로직
    ├── mod.rs
    ├── image_service.rs     # 이미지 처리 서비스
    └── icon_generator.rs    # 아이콘 생성 서비스
tests/
└── integration_tests.rs    # 통합 테스트
```

### 아키텍처

이 프로젝트는 trait 기반 의존성 주입을 사용한 모듈화된 아키�ecture를 채택합니다:

- **ImageProcessor**: 이미지 검증 및 크기 조정
- **IconGenerator**: 플랫폼별 아이콘 생성 로직
- **CliInterface**: 명령줄 인터페이스 처리

## 📋 명령어 옵션

```text
Android와 iOS용 앱 아이콘을 생성합니다

Usage: appicon_generator [OPTIONS] --input <IMAGE_PATH>

Options:
  -i, --input <IMAGE_PATH>   입력 이미지 파일 경로
  -o, --output <OUTPUT_DIR>  출력 디렉토리 경로 (기본값: 현재 디렉토리) [default: .]
  -h, --help                 Print help
  -V, --version              Print version
```

## 📸 예제

### 1. 기본 사용법

```bash
# logo.png에서 모든 아이콘 생성
appicon_generator --input logo.png --output ./app_icons
```

### 2. 현재 디렉토리에 생성

```bash
appicon_generator --input my_icon.png
```

### 3. 절대 경로 사용

```bash
appicon_generator --input /path/to/source.png --output /path/to/icons
```

## 🧪 테스트

프로젝트에는 포괄적인 테스트 스위트가 포함되어 있습니다:

```bash
# 모든 테스트 실행 (유닛 + 통합)
make test

# 테스트 커버리지 확인
make test-coverage

# 특정 테스트만 실행
cargo test test_android_icon_generation
```

## 📄 지원 파일 형식

### 입력 형식

- PNG
- JPEG/JPG
- BMP
- TIFF
- WEBP

### 출력 형식

- PNG (모든 아이콘이 PNG로 생성됩니다)

## ⚙️ 기술 스택

- **언어**: Rust (2024 Edition)
- **CLI**: clap 4.5.51
- **이미지 처리**: image 0.25.8
- **테스트**: tempfile, assert_fs, predicates

## 🤝 기여하기

1. 이 저장소를 포크합니다
2. 기능 브랜치를 생성합니다 (`git checkout -b feature/amazing-feature`)
3. 변경사항을 커밋합니다 (`git commit -m 'Add amazing feature'`)
4. 브랜치에 푸시합니다 (`git push origin feature/amazing-feature`)
5. Pull Request를 열어주세요

## 📝 라이선스

이 프로젝트는 MIT 라이선스 하에 배포됩니다. 자세한 내용은 [LICENSE](LICENSE) 파일을 참조하세요.

## 🔗 관련 링크

- [Android 아이콘 가이드라인](https://developer.android.com/guide/practices/ui_guidelines/icon_design_launcher)
- [iOS 아이콘 가이드라인](https://developer.apple.com/design/human-interface-guidelines/ios/icons-and-images/app-icon/)
- [Rust 공식 문서](https://doc.rust-lang.org/)

## 📞 지원

문제가 발생하거나 기능 요청이 있으시면 [Issues](../../issues) 페이지에서 알려주세요.

---

⭐ 이 프로젝트가 도움이 되었다면 별표를 눌러주세요!
