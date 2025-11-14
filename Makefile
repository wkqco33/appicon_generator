# 🔧 Makefile for App Icon Generator
# Android와 iOS용 앱 아이콘 생성기 빌드 및 관리 도구

# 기본 설정
CARGO := cargo
BINARY_NAME := appicon_generator
TARGET_DIR := target
BUILD_DIR := $(TARGET_DIR)/debug
RELEASE_DIR := $(TARGET_DIR)/release

# 색상 정의
RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[0;33m
BLUE := \033[0;34m
MAGENTA := \033[0;35m
CYAN := \033[0;36m
NC := \033[0m # No Color

# 기본 타겟
.PHONY: help
help: ## 📋 사용 가능한 명령어 목록을 보여줍니다
	@echo "$(CYAN)🚀 App Icon Generator - 사용 가능한 명령어$(NC)"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "$(BLUE)%-15s$(NC) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(YELLOW)💡 팁: 개발 시작은 'make setup' 명령어로 시작하세요!$(NC)"

# 개발 환경 설정
.PHONY: setup
setup: ## 🔧 개발 환경을 설정합니다 (종속성 설치 + 빌드)
	@echo "$(GREEN)🔧 개발 환경 설정 중...$(NC)"
	@$(MAKE) deps
	@$(MAKE) build
	@echo "$(GREEN)✅ 개발 환경 설정 완료!$(NC)"

.PHONY: deps
deps: ## 📦 Rust 종속성을 확인하고 설치합니다
	@echo "$(BLUE)📦 종속성 확인 중...$(NC)"
	@$(CARGO) check
	@echo "$(GREEN)✅ 종속성 확인 완료$(NC)"

# 빌드 관련
.PHONY: build
build: ## 🔨 개발용 빌드를 수행합니다
	@echo "$(BLUE)🔨 개발용 빌드 중...$(NC)"
	@$(CARGO) build
	@echo "$(GREEN)✅ 빌드 완료: $(BUILD_DIR)/$(BINARY_NAME)$(NC)"

.PHONY: release
release: ## 🚀 릴리스 빌드를 수행합니다 (최적화)
	@echo "$(MAGENTA)🚀 릴리스 빌드 중...$(NC)"
	@$(CARGO) build --release
	@echo "$(GREEN)✅ 릴리스 빌드 완료: $(RELEASE_DIR)/$(BINARY_NAME)$(NC)"

.PHONY: install
install: release ## 📥 시스템에 바이너리를 설치합니다
	@echo "$(MAGENTA)📥 시스템에 설치 중...$(NC)"
	@$(CARGO) install --path .
	@echo "$(GREEN)✅ 설치 완료! 이제 '$(BINARY_NAME)' 명령어를 사용할 수 있습니다$(NC)"

# 테스트 관련
.PHONY: test
test: ## 🧪 모든 테스트를 실행합니다
	@echo "$(BLUE)🧪 모든 테스트 실행 중...$(NC)"
	@$(CARGO) test
	@echo "$(GREEN)✅ 모든 테스트 통과!$(NC)"

.PHONY: test-unit
test-unit: ## 🔬 유닛 테스트만 실행합니다
	@echo "$(BLUE)🔬 유닛 테스트 실행 중...$(NC)"
	@$(CARGO) test --lib
	@echo "$(GREEN)✅ 유닛 테스트 통과!$(NC)"

.PHONY: test-integration
test-integration: ## 🔗 통합 테스트만 실행합니다
	@echo "$(BLUE)🔗 통합 테스트 실행 중...$(NC)"
	@$(CARGO) test --test integration_tests
	@echo "$(GREEN)✅ 통합 테스트 통과!$(NC)"

.PHONY: test-coverage
test-coverage: ## 📊 테스트 커버리지를 확인합니다
	@echo "$(BLUE)📊 테스트 커버리지 확인 중...$(NC)"
	@$(CARGO) test --verbose
	@echo "$(YELLOW)💡 자세한 커버리지는 tarpaulin 등의 도구를 사용하세요$(NC)"

# 개발 도구
.PHONY: dev
dev: build ## 🛠️ 개발 모드로 실행합니다 (예제 이미지로)
	@echo "$(CYAN)🛠️ 개발 모드 실행...$(NC)"
	@$(MAKE) example

.PHONY: run
run: build ## ▶️ 빌드된 바이너리를 실행합니다
	@echo "$(CYAN)▶️ 애플리케이션 실행 중...$(NC)"
	@./$(BUILD_DIR)/$(BINARY_NAME) --help

.PHONY: example
example: build ## 🎯 예제를 실행합니다 (테스트 이미지 생성)
	@echo "$(CYAN)🎯 예제 실행 중...$(NC)"
	@mkdir -p example_output
	@if [ ! -f "test_icon.png" ]; then \
		echo "$(YELLOW)📸 테스트 이미지 생성 중...$(NC)"; \
		python3 -c "from PIL import Image; img = Image.new('RGB', (512, 512), 'blue'); img.save('test_icon.png')" 2>/dev/null || \
		echo "$(RED)❌ PIL이 설치되지 않음. 수동으로 test_icon.png를 생성하세요$(NC)"; \
	fi
	@if [ -f "test_icon.png" ]; then \
		echo "$(GREEN)🚀 아이콘 생성 중...$(NC)"; \
		./$(BUILD_DIR)/$(BINARY_NAME) --input test_icon.png --output example_output; \
		echo "$(GREEN)✅ 예제 완료! example_output/ 폴더를 확인하세요$(NC)"; \
	fi

# 코드 품질
.PHONY: fmt
fmt: ## 🎨 코드를 포맷팅합니다
	@echo "$(BLUE)🎨 코드 포맷팅 중...$(NC)"
	@$(CARGO) fmt
	@echo "$(GREEN)✅ 코드 포맷팅 완료$(NC)"

.PHONY: lint
lint: ## 🔍 코드를 린팅합니다 (clippy)
	@echo "$(BLUE)🔍 코드 린팅 중...$(NC)"
	@$(CARGO) clippy -- -D warnings
	@echo "$(GREEN)✅ 린팅 완료$(NC)"

.PHONY: check
check: fmt lint test ## ✅ 모든 코드 품질 검사를 수행합니다
	@echo "$(GREEN)✅ 모든 검사 통과!$(NC)"

# 문서화
.PHONY: doc
doc: ## 📚 문서를 생성합니다
	@echo "$(BLUE)📚 문서 생성 중...$(NC)"
	@$(CARGO) doc --open
	@echo "$(GREEN)✅ 문서 생성 완료$(NC)"

.PHONY: doc-private
doc-private: ## 📖 비공개 항목을 포함한 문서를 생성합니다
	@echo "$(BLUE)📖 전체 문서 생성 중...$(NC)"
	@$(CARGO) doc --document-private-items --open
	@echo "$(GREEN)✅ 전체 문서 생성 완료$(NC)"

# 정리 및 유지보수
.PHONY: clean
clean: ## 🧹 빌드 아티팩트를 정리합니다
	@echo "$(YELLOW)🧹 정리 중...$(NC)"
	@$(CARGO) clean
	@rm -rf example_output
	@rm -f test_icon.png test_images
	@echo "$(GREEN)✅ 정리 완료$(NC)"

.PHONY: clean-all
clean-all: clean ## 🗑️ 모든 생성된 파일을 정리합니다
	@echo "$(YELLOW)🗑️ 전체 정리 중...$(NC)"
	@rm -rf test_output
	@find . -name "*.png" -not -path "./example_images/*" -delete 2>/dev/null || true
	@echo "$(GREEN)✅ 전체 정리 완료$(NC)"

# 배포 준비
.PHONY: package
package: clean release test ## 📦 배포용 패키지를 준비합니다
	@echo "$(MAGENTA)📦 배포 패키지 준비 중...$(NC)"
	@mkdir -p dist
	@cp $(RELEASE_DIR)/$(BINARY_NAME) dist/
	@cp README.md dist/
	@cp Cargo.toml dist/
	@echo "$(GREEN)✅ 배포 패키지 준비 완료: dist/$(NC)"

.PHONY: benchmark
benchmark: release ## ⚡ 성능 벤치마크를 실행합니다
	@echo "$(MAGENTA)⚡ 성능 벤치마크 실행 중...$(NC)"
	@if [ -f "test_icon.png" ]; then \
		echo "$(BLUE)🔥 릴리스 빌드로 성능 측정...$(NC)"; \
		time ./$(RELEASE_DIR)/$(BINARY_NAME) --input test_icon.png --output benchmark_output; \
		rm -rf benchmark_output; \
	else \
		echo "$(RED)❌ test_icon.png가 없습니다. 'make example'을 먼저 실행하세요$(NC)"; \
	fi

# 정보 표시
.PHONY: info
info: ## ℹ️ 프로젝트 정보를 표시합니다
	@echo "$(CYAN)📊 프로젝트 정보$(NC)"
	@echo "$(BLUE)프로젝트명:$(NC) App Icon Generator"
	@echo "$(BLUE)언어:$(NC) Rust ($(shell rustc --version 2>/dev/null || echo 'Rust 미설치'))"
	@echo "$(BLUE)Cargo 버전:$(NC) $(shell cargo --version 2>/dev/null || echo 'Cargo 미설치')"
	@echo "$(BLUE)바이너리명:$(NC) $(BINARY_NAME)"
	@echo "$(BLUE)빌드 디렉토리:$(NC) $(BUILD_DIR)"
	@echo "$(BLUE)릴리스 디렉토리:$(NC) $(RELEASE_DIR)"
	@echo ""
	@if [ -f "$(BUILD_DIR)/$(BINARY_NAME)" ]; then \
		echo "$(GREEN)✅ 개발용 빌드 존재$(NC)"; \
	else \
		echo "$(RED)❌ 개발용 빌드 없음 - 'make build' 실행 필요$(NC)"; \
	fi
	@if [ -f "$(RELEASE_DIR)/$(BINARY_NAME)" ]; then \
		echo "$(GREEN)✅ 릴리스 빌드 존재$(NC)"; \
	else \
		echo "$(RED)❌ 릴리스 빌드 없음 - 'make release' 실행 필요$(NC)"; \
	fi

.PHONY: version
version: ## 🏷️ 현재 버전을 표시합니다
	@echo "$(CYAN)🏷️ 현재 버전$(NC)"
	@grep "version" Cargo.toml | head -1 | cut -d'"' -f2

# 기본 타겟을 help로 설정
.DEFAULT_GOAL := help

# 파일이 아닌 타겟들을 명시
.PHONY: help setup deps build release install test test-unit test-integration test-coverage \
        dev run example fmt lint check doc doc-private clean clean-all package benchmark info version