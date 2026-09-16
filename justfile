# justfile — единая точка входа для типовых задач разработки (см. docs/development.md).
#
# Требуется: cargo-leptos (just setup), npx — только для e2e.
# Tailwind CSS отдельно ставить не нужно: cargo-leptos сам скачивает
# standalone-бинарник (см. tailwind-input-file в Cargo.toml).
#
# Список рецептов: just --list

# Показать доступные рецепты.
default:
	@just --list

# Подготовка окружения (Linux/macOS): Docker, Node, Rust, wasm, cargo-leptos.
setup:
	bash scripts/setup.sh

# Dev-сервер с live-reload (127.0.0.1:3000).
watch:
	cargo leptos watch

# Релизная сборка: сервер (ssr) + WASM (hydrate) + CSS → target/site.
build:
	cargo leptos build --release

# Запуск релизной сборки локально (отдаёт target/site).
serve:
	cargo leptos serve

# Форматирование.
fmt:
	cargo fmt --all

# Проверка форматирования (как в CI).
fmt-check:
	cargo fmt --all -- --check

# Линтер строго, как в CI.
lint:
	cargo clippy --features ssr --all-targets -- -D warnings
	cargo clippy -p core-shared --all-targets -- -D warnings

# Юнит-тесты + гейт компиляции клиента под wasm32.
test:
	cargo test --features ssr --lib
	cargo test -p core-shared
	cargo check --features hydrate --lib --target wasm32-unknown-unknown

# Rustdoc без зависимостей.
doc:
	cargo doc --features ssr --no-deps

# Аудит уязвимостей зависимостей.
audit:
	cargo audit

# E2E (Playwright). Нужен запущенный сервер: just watch или just serve.
# Локально можно выбрать движок: just e2e --project=firefox
e2e *ARGS:
	cd end2end && npx playwright test {{ARGS}}

# Docker-образ + запуск через compose (http://localhost:3000).
docker:
	docker compose -f deploy/docker-compose.yaml up --build

# Полный прогон проверок перед коммитом.
precommit: fmt lint test doc
	@echo "✅ precommit: fmt + clippy + tests + rustdoc пройдены"

# Очистка артефактов сборки.
clean:
	cargo clean
