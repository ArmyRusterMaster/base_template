# Roadmap и план развития

База для производных шаблонов (микросаас и др.). Бизнес-слой (auth, оплата,
коннекторы к внешним сервисам, devcontainer, CD) — в шаблоне микросааса, здесь
только каркас.

Статусы: `done` | `in_progress` | `planned`. Обновляется вместе с
[current-state.md](current-state.md).

## Сделано (фазы 1–11)

### Фаза 1. just + setup.sh — **done**

- [x] `justfile` (watch/build/serve/fmt/fmt-check/lint/test/doc/audit/e2e/docker/
  precommit/clean).
- [x] `scripts/setup.sh`: cargo-binstall, `just`, `git-cliff`, `rustfmt`/`clippy`,
  npm-зависимости e2e.
- [x] Осознанное отклонение: **tailwind CLI не ставится** — standalone-бинарник
  скачивает cargo-leptos (`LEPTOS_TAILWIND_VERSION`), глобальная версия могла бы
  разойтись с ожидаемой (см. [asset-pipeline.md](asset-pipeline.md)).

### Фаза 2. Observability — **done**

- [x] `tracing` + `tracing-subscriber(env-filter)`, `src/logging.rs`
  (`RUST_LOG` → `APP_LOG_LEVEL` → `info`), поле `log_level` в конфиге.
- [x] request-id middleware (`tower-http`), `x-request-id` в ответе; закрыт
  e2e-тестом.

### Фаза 3. Инфраструктурный AppError — **done**

- [x] `src/error.rs`: `Config`/`NotFound`/`Internal`, `IntoResponse` (JSON без
  деталей наружу), `From<ConfigError>`, юнит-тесты.

### Фаза 4. Трейты коннекторов — **done**

- [x] `src/connectors/`: трейт `Connector` (id/description/health),
  `ConnectorRegistry`, демо `EchoConnector`, `GET /api/health` агрегирует статусы.

### Фаза 5. Tailwind вместо SCSS — **done**

- [x] `style/main.css` (Tailwind v4, CSS-first: `@import "tailwindcss"`,
  `@source "../src"`, `@theme`), `style/main.scss` удалён.
- [x] `Cargo.toml`: `tailwind-input-file = "style/main.css"` вместо `style-file`;
  результат — `target/site/pkg/base_template.css` (тот же href в `src/app.rs`).

### Фаза 6. UI-скелет — **done**

- [x] `<Layout>`: шапка (логотип, версия, меню), сайд-меню (адаптив,
  `use_media_query`), футер (copyright + версия).
- [x] Страницы: Главная, Личный кабинет (профиль-заглушка + «Подключения» из
  реестра), Статус сервисов, 404.
- [x] `leptos_use` подключён; `thaw-ui` **не используется**: 0.4.x требует
  leptos 0.7 (несовместим с 0.8) — удалён из зависимостей.

### Фаза 7. Воркспейс `core-shared` — **done**

- [x] Корень → `[workspace] members = ["crates/core-shared"]`, `resolver = "2"`.
- [x] `crates/core-shared` — DTO, общие для SSR и WASM (`HealthResponse`,
  `ConnectorHealthDto`), только `serde` (без leptos/axum/tokio), юнит-тесты
  контракта JSON.
- [x] `src/dto.rs` удалён, приложение импортирует `core_shared::*`.
- [x] CI/just: `cargo test -p core-shared`, `cargo clippy -p core-shared ...`.
- [x] Docs обновлены (architecture + раздел «Воркспейс», development, AGENTS).
- [ ] Решение: `AppInfo` **не создавали** — в шаблоне нет потребителя, тип не
  должен появляться «на будущее» (добавляется в наследниках вместе с запросом).
- [x] Сборка подтверждена локально (16.09.2026): воркспейс-метадата корректна,
  `cargo check/test/clippy/doc` и wasm-гейт зелёные. Полный `cargo leptos build`
  (Tailwind + wasm-pack) проверяется в CI (джоба `build-release`);
  `[[workspace.metadata.leptos]]` намеренно не добавлен.

### Фаза 8. CI-оптимизации — **done**

- [x] cargo-binstall вместо `cargo install cargo-leptos` (CI и `deploy/Dockerfile`,
  с откатом на `cargo install`).
- [x] WASM-гейт в джобе `test`:
  `cargo check --features hydrate --lib --target wasm32-unknown-unknown`.
- [x] E2E в CI — только chromium; ожидание готовности сервера вместо `sleep`.
- [x] Отдельная джоба `audit` (`cargo audit`).

### Фаза 9. Версионирование + авто-changelog — **done**

- [x] Формат `vX.Y.Z-<hash>`: `build.rs` (`GIT_HASH` env → `git rev-parse` →
  `dev`), `src/version.rs`, версия в UI, `/api/health`, логе старта.
- [x] `cliff.toml` + CI-джоба `release` по тегу `v*` (CHANGELOG.md → GitHub
  Release), см. [versioning.md](versioning.md).

### Фаза 10. Устойчивый e2e — **done**

- [x] `BASE_URL` из env (`end2end/playwright.config.ts`), константы приложения в
  `end2end/constants.ts` (имя приложения/заголовок переопределяются env).
- [x] Тесты `/`, навигация `/account`/`/status`, 404, smoke `/api/health`
  (+ `x-request-id`).

### Фаза 11. Документация — **done**

- [x] stack/architecture/configuration/development/deployment/asset-pipeline/
  best-practices/current-state/roadmap/README + `docs/versioning.md` +
  `docs/agents/`.

## Осталось

### Технический долг — **done (16.09.2026)**

- [x] `Cargo.lock` пересобран и актуален (удалены `thaw`/`leptos-fetch`, добавлен
  path-крейт `core-shared`) — обновлён при сборках и включён в рабочий diff.
- [x] Локальный прогон полного набора проверок на машине с toolchain (см.
  current-state.md → Last verified): `cargo fmt --all -- --check`,
  `cargo check --features ssr`, `cargo clippy --features ssr --all-targets -- -D warnings`,
  `cargo test --features ssr` (10 passed), `cargo test -p core-shared` (3 passed),
  wasm-гейт `cargo check --features hydrate --lib --target wasm32-unknown-unknown`,
  `cargo doc --workspace --no-deps`. Всё зелёное.

## Среднесрочные

- [x] cargo-audit в отдельной джобе.
- [ ] Тег Docker-образа с версией (`vX.Y.Z-<hash>`) и публикация в реестр (CD).
- [ ] Compile-time конфиг (serde + `include_str!`).
- [ ] Миграция `serde_yaml` → `serde_yml`/`yaml-rust2`.
- [ ] Отдельный `cargo chef cook` для WASM-зависимостей (`--target-dir target/front`).
- [ ] Timeout/retry для клиентских запросов к API.

## Шаблон микросааса (наследник) — НЕ в этой базе

Auth, оплата, конкретные коннекторы (Stripe/Telegram/почта), роли,
devcontainer, CD-пайплайн, бизнес-домены.