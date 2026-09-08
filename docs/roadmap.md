# Roadmap и план развития

База для производных шаблонов (микросаас и др.). Бизнес-слой (auth, оплата,
коннекторы к внешним сервисам, devcontainer, CD) — в шаблоне микросааса, здесь
только каркас.

## Сделано

- [x] Graceful shutdown (Ctrl+C / SIGTERM).
- [x] Скрипт окружения `scripts/setup.sh` (Docker, Node, Rust+wasm, cargo-leptos,
  leptosfmt, cargo-audit; sccache сознательно не ставится).
- [x] Типизированная конфигурация: YAML (`config/app.yaml`) + env `APP_*`,
  слои + fail-fast валидация, юнит-тесты.
- [x] Docker: мультистейдж (Ubuntu→Alpine, cargo-chef, musl-статик) +
  `Dockerfile.prebuilt` для CI + docker-compose с healthcheck.
- [x] CI-конвейер: fmt(автокоммит) ∥ clippy → test → build-release(артефакт) →
  e2e ∥ docker → smoke.
- [x] sccache отключён локально (`.cargo/config.toml`, `rustc-wrapper = ""`).
- [x] Документация `docs/`, `docs/current-state.md`.

## План улучшения (v3, поэтапно)

### Фаза 1. just + setup.sh — `justfile` (watch/build/fmt/lint/test/e2e/docker/`precommit`); в setup.sh: установка `just`, `tailwind` CLI (если нет), `git-cliff` (опционально, через binstall).

### Фаза 2. Observability — `tracing` + `tracing-subscriber(env-filter)`;
`src/logging.rs` (RUST_LOG → APP_LOG_LEVEL → info); поле `log_level` в конфиге;
request-id middleware (tower-http `request-id`); замена `log!`/`println!`.

### Фаза 3. Инфраструктурный AppError — `src/error.rs`: Config/Internal/NotFound,
`IntoResponse` (JSON, без деталей наружу), `From`-конверсии, тесты. Внутренний
каркас: наследники маппят в доменные ошибки.

### Фаза 4. Трейты коннекторов — `src/connectors/`: trait `Connector`
(id/description/health), `ConnectorRegistry`, демо `EchoConnector`,
`/api/health` агрегирует статусы. Наследники добавляют конкретные интеграции.

### Фаза 5. Tailwind вместо SCSS — `style/main.css` + tailwind через
cargo-leptos (версию сверить), обновить метаданные и доки.

### Фаза 6. UI-скелет — `<Layout>`: шапка (логотип, версия, меню),
сайд-меню (адаптив, `use_media_query`), футер (copyright + версия); страницы:
Главная, Личный кабинет (профиль-заглушка + «Подключения» из реестра),
Статус сервисов, 404. thaw-ui (если совместим с leptos 0.8), leptos_use,
leptos_fetch.

### Фаза 7. Воркспейс `core-shared` — корень → `[workspace]`,
`crates/core-shared` (DTO: AppInfo, HealthResponse, ConnectorStatusDto);
проверить cargo-leptos в воркспейсе.

### Фаза 8. CI-оптимизации — cargo-binstall вместо `cargo install cargo-leptos`;
WASM-гейт в `test` (`cargo check --features hydrate --target wasm32`);
эксперимент: chef cook для front-депов (`--target-dir target/front`).

### Фаза 9. Версионирование + авто-changelog — формат `vX.Y.Z-<hash>`;
`build.rs` (GIT_HASH env → git rev-parse → "dev", усечение до 7 символов);
`src/version.rs`; версия в футере, `/api/health`, логе старта, теге образа;
`cliff.toml` + CI-джоба `release` (git-cliff → CHANGELOG.md → GitHub Release
по тегу `vX.Y.Z`); conventional commits обязательны (см. RULES.md §6).

### Фаза 10. Устойчивый e2e — `BASE_URL` из env; тесты `/`, `/account`,
`/status`, smoke `/api/health`; тайтл из env/константы.

### Фаза 11. Документация — stack/architecture/development/asset-pipeline/
current-state/README + `docs/versioning.md`.

## Среднесрочные (после фаз)

- [ ] cargo-audit в отдельной джобе.
- [ ] compile-time конфиг (serde + include_str!).
- [ ] CD: публикация образа в реестр (для наследников).

## Шаблон микросааса (наследник) — НЕ в этой базе

Auth, оплата, конкретные коннекторы (Stripe/Telegram/почта), роли,
devcontainer, CD-пайплайн, бизнес-домены.