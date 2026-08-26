# App Icon Generator

하나의 이미지에서 Android와 iOS 앱 아이콘을 생성하는 Rust CLI입니다.

## 기능

- Android 5개 밀도: `mdpi`부터 `xxxhdpi`까지
- iOS 15개 용도: 알림, 설정, Spotlight, 앱 아이콘, App Store
- Lanczos3 리샘플링과 PNG 출력
- 입력 파일의 확장자와 실제 이미지 데이터 검증

## 설치

### Cargo

```bash
cargo install --git https://github.com/wkqco33/appicon_generator appicon_generator
```

### PPM

GitHub Release가 게시되면 PPM이 OS와 아키텍처에 맞는 사전 빌드 바이너리를 설치합니다.

```bash
ppm install wkqco33/appicon_generator
```

릴리스 자산은 `appicon_generator_{os}_{arch}.tar.gz` 또는 Windows용 `.zip`이며, 각 자산에 SHA-256 체크섬 파일이 함께 제공됩니다.

## 사용법

```bash
appicon_generator --input logo.png --output ./app_icons
appicon_generator --input logo.png
appicon_generator --help
```

입력은 PNG, JPEG, GIF, BMP, TIFF, WEBP를 지원합니다. 출력 디렉토리의 구조는 다음과 같습니다.

```text
app_icons/
├── android/
│   ├── drawable-mdpi/ic_launcher.png
│   ├── drawable-hdpi/ic_launcher.png
│   ├── drawable-xhdpi/ic_launcher.png
│   ├── drawable-xxhdpi/ic_launcher.png
│   └── drawable-xxxhdpi/ic_launcher.png
└── ios/
    ├── Icon-App-20x20@1x.png
    ├── Icon-App-60x60@3x.png
    └── Icon-App-1024x1024@1x.png
```

## 개발

요구사항: Rust stable, Cargo, 선택적으로 `just`.

```bash
cargo test --all-targets
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

`just check`는 포맷, 린트, 테스트를 순서대로 실행합니다. 전체 명령은 `just --list`에서 확인할 수 있습니다.

구조:

- `src/cli.rs`: CLI 파싱
- `src/models/`: 설정과 플랫폼별 아이콘 메타데이터
- `src/services/`: 이미지 처리와 아이콘 생성
- `tests/`: 파일시스템을 포함한 통합 테스트

서비스는 `ImageProcessor`와 `IconGenerator` trait를 통해 의존성을 주입할 수 있습니다. 새 동작은 먼저 모듈 테스트 또는 통합 테스트를 추가하고 구현합니다. 상세한 규칙은 [AGENTS.md](AGENTS.md)를 참고하세요.

## 보안 및 기여

자격 증명, 개인정보, 생성물, 릴리스 아카이브를 커밋하지 마세요. 취약점은 공개 이슈가 아닌 [SECURITY.md](SECURITY.md)의 절차를 이용해 제보해 주세요. 기여 방법은 [CONTRIBUTING.md](CONTRIBUTING.md)에 있습니다.

## 라이선스

MIT License. 자세한 내용은 [LICENSE](LICENSE)를 참고하세요.
