# Текущее состояние проекта

Обновляется при изменениях. Последняя проверка: **15.09.2026**.

Разделы соответствуют правилам для агентов (см. также
[docs/agents/](agents/)).

## Implemented

- **Каркас Leptos SSR + hydration**: `src/app.rs` (shell + App + страницы),
  `src/main.rs` (axum-сервер), `src/lib.rs` (hydrate); маршруты `/`, `/account`,
  `/status`, 404 внутри layout.
- **Graceful shutdown** (Ctrl+C / SIGTERM) — `src/main.rs`.
- **Типизированная конфигурация** — `src/config.rs` + `config/app.yaml` +
  переменные `APP_*`: слои, fail-fast-валидация, юнит-тесты.
- **Observability** — `src/logging.rs` (`RUST_LOG` → `log_level` → `info`),
  middleware request-id (`tower-http`) в `src/main.rs`.
- **Инфраструктурный AppError** — `src/error.rs`: `Config`/`NotFound`/`Internal`,
  `IntoResponse` (JSON без внутренних деталей), `From<ConfigError>`, тесты.
- **Коннекторы** — `src/connectors/mod.rs`: трейт `Connector`, `ConnectorRegistry`,
  демо `EchoConnector`; `GET /api/health` агрегирует статусы.
- **UI-скелет** — `src/components.rs` (Layout: шапка с версией, сайд-меню с
  `use_media_query`, футер) + страницы; стилизация — Tailwind-утилиты в `view!`.
- **Tailwind CSS v4** — `style/main.css` (`@import "tailwindcss"`, `@source`,
  `@theme`), сборка через `cargo-leptos` (`tailwind-input-file`): cargo-leptos сам
  скачивает standalone-бинарник Tailwind, глобальный CLI не нужен.
- **Версионирование** — `build.rs` (`GIT_HASH`) + `src/version.rs`
  (`vX.Y.Z-<hash>`), версия в UI, `/api/health` и логе старта; `cliff.toml` —
  см. [versioning.md](versioning.md).
- **Docker** — мультистейдж `deploy/Dockerfile` (Ubuntu → Alpine, cargo-chef,
  cargo-binstall, cargo-leptos, musl-статик), `deploy/Dockerfile.prebuilt` для CI,
  `deploy/docker-compose.yaml` с healthcheck.
- **CI/CD** — `.github/workflows/ci.yaml`: fmt (+автокоммит) ∥ clippy ∥ audit →
  test (+ wasm-гейт) → build-release → e2e ∥ docker → smoke → release (по тегу).
- **justfile** — типовые задачи: `just watch/build/serve/fmt/lint/test/e2e/docker/precommit`.
- **Setup-скрипт** — `scripts/setup.sh`: Docker, Node, Rust + wasm, cargo-binstall,
  cargo-leptos, leptosfmt, just, cargo-audit, git-cliff, npm-зависимости e2e.
- **E2E** — `end2end/` (Playwright): константы и `BASE_URL` из env, тесты главной,
  навигации/404 и `/api/health` (см. [development.md](development.md)).
- **Документация** — `docs/` (+ `docs/agents/` — рабочие заметки агентов).

## In progress

- **Фаза 7 roadmap**: воркспейс `crates/core-shared` для общих DTO — пока DTO
  живут в `src/dto.rs`.

## Planned

- см. [roadmap.md](roadmap.md): воркспейс `core-shared`, compile-time конфиг,
  CD (публикация образа), тег Docker-образа с версией, миграция `serde_yaml`.

## Known limitations

- **Локальная компиляция в этой сессии не запускалась**: на рабочей машине нет
  доступного Rust-toolchain (`.cargo`/`.rustup` принадлежат другому пользователю
  Windows, `cargo`/`rustup` отсутствуют в `PATH`). Правки проверялись чтением
  кода против фактических API `leptos 0.8.20` / `leptos_router 0.8.15`, а также
  валидацией конфигов и TypeScript. Фактическая сборка проверяется в CI.
- `Cargo.lock` не пересобран после удаления `thaw`/`leptos-fetch` (зависимости
  больше не используются и выпадут из lock при первой сборке). Рекомендуется
  выполнить `cargo build` (любой) и закоммитить обновлённый lock.
- **Tailwind** требует сети на этапе сборки (cargo-leptos скачивает бинарник).
- **Docker локально не проверен** (нет Docker CLI) — сборка образа проверяется
  в CI (джоба `docker`).
- **E2E в CI — только chromium** (`npx playwright test --project=chromium`);
  firefox/webkit доступны локально.
- `serde_yaml` помечен deprecated на crates.io — миграция на `serde_yml`/
  `yaml-rust2` запланирована.
- **sccache отключён локально** (`.cargo/config.toml`, `rustc-wrapper = ""`) —
  sccache падает на Windows при компиляции `web-sys`.
- WASM-зависимости в Docker кешируются не полностью (возможен отдельный
  `cargo chef cook --features hydrate`).
- `thaw-ui` (0.4.x) требует `leptos 0.7` → с 0.8 несовместим и удалён из
  зависимостей; UI строится на Tailwind-утилитах и `leptos-use`.

## Open risks

- `leptos_router::components::A` в 0.8 не принимает `class` как prop — стили
  передаются через attribute spreading (`attr:class`). При обновлении
  `leptos_router` синтаксис нужно перепроверить.
- Клиентский запрос `/api/health` (`gloo-net`) идёт без таймаута и retry —
  наследникам стоит добавить timeout/backpressure.
- Джоба `audit` может краснеть из-за новых advisory в транзитивных зависимостях;
  это ожидаемое поведение, но требует ручного разбора.
- `git-cliff`-джоба коммитит `CHANGELOG.md` в `main` из сборки по тегу: если
  `main` успел уйти вперёд, пуш не выполнится (в логе будет предупреждение).

## Last verified

- **15.09.2026**: `end2end` — `npm install` + `npx tsc --noEmit` → успешно (exit 0).
- **15.09.2026**: `.github/workflows/ci.yaml` разобран как YAML (9 джоб),
  `cliff.toml` и `Cargo.toml` — как TOML; структура джоб проверена.
- **08.09.2026** (прошлая сессия): `cargo check` падал с ошибками компиляции
  (импорты `leptos_router`, `spawn`, `class` у `<A>`); они исправлены,
  повторный `cargo check` не выполнялся (нет toolchain) — см.
  [agents/change-log.md](agents/change-log.md).