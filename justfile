# App Icon Generator development commands

set shell := ["bash", "-uc"]

cargo := "cargo"
binary_name := "appicon_generator"
build_dir := "target/debug"
release_dir := "target/release"

# Show available commands with `just --list`.
help:
    @echo "App Icon Generator - 사용 가능한 명령어"
    @just --list
    @echo ""
    @echo "팁: 개발 시작은 'just setup' 명령어로 시작하세요!"

setup: deps build
    @echo "개발 환경 설정 완료!"

deps:
    @echo "종속성 확인 중..."
    @{{ cargo }} check
    @echo "종속성 확인 완료"

build:
    @echo "개발용 빌드 중..."
    @{{ cargo }} build
    @echo "빌드 완료: {{ build_dir }}/{{ binary_name }}"

release:
    @echo "릴리스 빌드 중..."
    @{{ cargo }} build --release
    @echo "릴리스 빌드 완료: {{ release_dir }}/{{ binary_name }}"

install: release
    @echo "시스템에 설치 중..."
    @{{ cargo }} install --path .
    @echo "설치 완료! 이제 '{{ binary_name }}' 명령어를 사용할 수 있습니다"

test:
    @echo "모든 테스트 실행 중..."
    @{{ cargo }} test
    @echo "모든 테스트 통과!"

test-unit:
    @echo "유닛 테스트 실행 중..."
    @{{ cargo }} test --lib
    @echo "유닛 테스트 통과!"

test-integration:
    @echo "통합 테스트 실행 중..."
    @{{ cargo }} test --test integration_tests
    @echo "통합 테스트 통과!"

test-coverage:
    @echo "테스트 커버리지 확인 중..."
    @{{ cargo }} test --verbose
    @echo "자세한 커버리지는 tarpaulin 등의 도구를 사용하세요"

dev: build
    @echo "개발 모드 실행..."
    @just example

run: build
    @echo "애플리케이션 실행 중..."
    @./{{ build_dir }}/{{ binary_name }} --help

example: build
    @echo "예제 실행 중..."
    @mkdir -p example_output
    @if [ ! -f "test_icon.png" ]; then \
        echo "테스트 이미지 생성 중..."; \
        python3 -c "from PIL import Image; img = Image.new('RGB', (512, 512), 'blue'); img.save('test_icon.png')" 2>/dev/null || \
        echo "PIL이 설치되지 않음. 수동으로 test_icon.png를 생성하세요"; \
    fi
    @if [ -f "test_icon.png" ]; then \
        echo "아이콘 생성 중..."; \
        ./{{ build_dir }}/{{ binary_name }} --input test_icon.png --output example_output; \
        echo "예제 완료! example_output/ 폴더를 확인하세요"; \
    fi

fmt:
    @echo "코드 포맷팅 중..."
    @{{ cargo }} fmt
    @echo "코드 포맷팅 완료"

lint:
    @echo "코드 린팅 중..."
    @{{ cargo }} clippy --all-targets --all-features -- -D warnings
    @echo "린팅 완료"

check:
    @{{ cargo }} fmt --all -- --check
    @{{ cargo }} clippy --all-targets --all-features -- -D warnings
    @{{ cargo }} test --all-targets
    @echo "모든 검사 통과!"

doc:
    @echo "문서 생성 중..."
    @{{ cargo }} doc --open
    @echo "문서 생성 완료"

doc-private:
    @echo "전체 문서 생성 중..."
    @{{ cargo }} doc --document-private-items --open
    @echo "전체 문서 생성 완료"

clean:
    @echo "정리 중..."
    @{{ cargo }} clean
    @rm -rf example_output
    @rm -f test_icon.png test_images
    @echo "정리 완료"

clean-all: clean
    @echo "전체 정리 중..."
    @rm -rf test_output
    @find . -name "*.png" -not -path "./example_images/*" -delete 2>/dev/null || true
    @echo "전체 정리 완료"

package: clean release test
    @echo "배포 패키지 준비 중..."
    @mkdir -p dist
    @cp {{ release_dir }}/{{ binary_name }} dist/
    @cp README.md dist/
    @cp Cargo.toml dist/
    @echo "배포 패키지 준비 완료: dist/"

benchmark: release
    @if [ -f "test_icon.png" ]; then \
        echo "릴리스 빌드로 성능 측정..."; \
        time ./{{ release_dir }}/{{ binary_name }} --input test_icon.png --output benchmark_output; \
        rm -rf benchmark_output; \
    else \
        echo "test_icon.png가 없습니다. 'just example'을 먼저 실행하세요"; \
    fi

info:
    @echo "프로젝트 정보"
    @echo "프로젝트명: App Icon Generator"
    @echo "언어: Rust ($(rustc --version 2>/dev/null || echo 'Rust 미설치'))"
    @echo "Cargo 버전: $({{ cargo }} --version 2>/dev/null || echo 'Cargo 미설치')"
    @echo "바이너리명: {{ binary_name }}"
    @echo "빌드 디렉토리: {{ build_dir }}"
    @echo "릴리스 디렉토리: {{ release_dir }}"
    @if [ -f "{{ build_dir }}/{{ binary_name }}" ]; then echo "개발용 빌드 존재"; else echo "개발용 빌드 없음 - 'just build' 실행 필요"; fi
    @if [ -f "{{ release_dir }}/{{ binary_name }}" ]; then echo "릴리스 빌드 존재"; else echo "릴리스 빌드 없음 - 'just release' 실행 필요"; fi

version:
    @echo "현재 버전"
    @grep "version" Cargo.toml | head -1 | cut -d'"' -f2
