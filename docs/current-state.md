# Текущее состояние проекта

Обновляется при изменениях. Последняя проверка: XX.XX.2026.

## Что уже реализовано

- **Каркас Leptos SSR + hydration**: `src/app.rs` (shell + App), `src/main.rs`
  (axum-сервер), `src/lib.rs` (hydrate), маршрут `/`.
- **Graceful shutdown** (Ctrl+C / SIGTERM) — в `main.rs`.
- **Типизированная конфигурация** — `src/config.rs` + `config/app.yaml` +
  переменные `APP_*`: слои, fail-fast-валидация, юнит-тесты.
- **Docker** — мультистейдж `deploy/Dockerfile` (Ubuntu → Alpine, cargo-chef,
  cargo-leptos, musl-статик) и `deploy/docker-compose.yaml` с healthcheck.
- **CI/CD** — `.github/workflows/ci.yaml` (fmt/clippy/test, релиз-сборка,
  e2e Playwright, Docker-image smoke).
- **Asset pipeline** — cache-busting через cargo-leptos (хэши в именах
  бандлов). Предсжатие `.gz`/`.br` убрано: при необходимости сжатие
  обеспечивают внешний слой (Cloudflare/Nginx/Caddy) — см.
  [asset-pipeline.md](asset-pipeline.md).
- **Документация** структурирована: `docs/`.
- **Setup-скрипт** окружения — `scripts/setup.sh`.

## Что проверено локально

- `cargo test --features ssr` — юнит-тесты конфигурации.
- Линтинг/форматирование — cargo fmt/clippy.

## Известные ограничения / TODO

- **Проверено без Docker**: на машине разработчика нет Docker CLI — сборка
  образа проверяется только в CI (джоба `docker`) и при первом локальном
  `docker compose up`.
- **E2E-тесты** рассчитаны на `http://localhost:3000` и title «Welcome to
  Leptos»: при смене заголовка/порта нужно обновить `end2end/tests/example.spec.ts`.
- **Cargo.lock** появился в работе (`cargo check`) — рекомендуется закоммитить
  для воспроизводимости; строка в `.gitignore` убрана.
- `serde_yaml` помечен как deprecated в crates.io (проект переезжает на
  `serde_yml`/`yaml-rust2`) — при обновлении провести миграцию в `src/config.rs`.
- WASM-зависимости (feature `hydrate`) в Docker кешируются не полностью —
  для них отдельный `cargo chef cook` с `--features hydrate` может заметно
  ускорить сборку (план: [roadmap.md](roadmap.md)).

## План ближайших шагов

См. [roadmap.md](roadmap.md).