# Текущее состояние проекта

Обновляется при изменениях. Последняя проверка: **16.09.2026**.

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
- **Воркспейс `crates/core-shared`** — DTO, общие для SSR и WASM
  (`HealthResponse`, `ConnectorHealthDto`), только `serde`; `src/dto.rs` удалён;
  тесты контракта JSON. См. [architecture.md](architecture.md) → «Воркспейс».
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

Активной задачи нет. Текущий статус и следующий шаг — в
[agents/current-task.md](agents/current-task.md), историческое состояние — в
[agents/change-log.md](agents/change-log.md).

## Planned

- см. [roadmap.md](roadmap.md): compile-time конфиг, CD (публикация образа),
  тег Docker-образа с версией, миграция `serde_yaml`, timeout/retry для
  клиентских запросов.

## Known limitations

- **Полная сборка бинарника и `cargo-leptos build` в этой сессии не запускались**:
  компиляция подтверждена `cargo check/clippy/test/doc` (см. Last verified), но
  связка cargo-leptos + Tailwind + wasm-pack (`just build`) не прогонялась.
- **Совместимость cargo-leptos с воркспейсом** подтверждена на уровне
  `cargo metadata`/check: `[package.metadata.leptos]` в корневом пакете даёт
  единственный проект; полный `cargo leptos build` проверяется в CI
  (джоба `build-release`).
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

- **16.09.2026**: локальный toolchain-прогон (Windows, stable MSVC, wasm-таргет
  добавлен): `cargo fmt --all -- --check` → ok; `cargo check --features ssr` →
  exit 0; `cargo clippy --features ssr --all-targets -- -D warnings` → ok (только
  шум LNK4044 от глобального конфига линкера пользователя и future-incompat
  заметка о зависимости `proc-macro-error2`); `cargo test --features ssr` →
  10 passed / 0 failed (включая тесты `core-shared`); `cargo test -p core-shared`
  → 3 passed / 0 failed; wasm-гейт `cargo check --features hydrate --lib --target
  wasm32-unknown-unknown` → ok; `cargo doc --workspace --no-deps` → ok;
  `Cargo.lock` пересобран. Фикс вывода типов в `src/main.rs`:
  `file_and_error_handler::<AppState, _>` (иначе `S` неоднозначен из-за
  рефлексивного `FromRef` impl axum).
- **16.09.2026**: Фаза 7 — создан `crates/core-shared` (воркспейс), DTO перенесены,
  `src/dto.rs` удалён; CI/just/docs обновлены.
- **16.09.2026**: `end2end` — `npm install` + `npx tsc --noEmit` → успешно (exit 0).