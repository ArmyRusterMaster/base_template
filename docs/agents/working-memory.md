# Working memory (устойчивый контекст для агентов)

Только долговременные соглашения, ограничения и ловушки. Временные рассуждения и
логи сюда не пишутся.

## Архитектурные инварианты

- **Разделение SSR/hydrate**: всё, что требует axum/tokio/файловой системы,
  компилируется только под `feature = "ssr"` (см. `src/lib.rs`). Клиентский бандл
  (WASM) не должен тянуть серверные зависимости.
- **Конфигурация — только через `config/app.yaml` + `APP_*`** и только через
  `AppConfig`; произвольные `std::env::var` в бизнес-коде не читаем. Валидация —
  fail-fast на старте (`AppConfig::load`).
- **Ошибки наружу — через `AppError`**: клиент получает безопасный JSON
  (`code`/`message`), детали идут только в лог (`tracing`).
- **Коннекторы — за трейтом `Connector`**: новый внешний сервис = файл с
  реализацией трейта + `registry.register(...)`; никакого доступа к внешним API
  из UI-кода напрямую.
- **Версия сборки** — единственный источник `src/version::version()`
  (`vX.Y.Z-<hash>` из `build.rs`); не хардкодить версию в UI/API.
- **DTO — только в `crates/core-shared`**: тип, нужный и серверу, и клиенту,
  живёт в крейте воркспейса (без зависимостей от `leptos`/`axum`/`tokio`), а не
  дублируется в `src/`. Публичный путь — `core_shared::<Type>`.
- **Воркспейс**: `[workspace] members = ["crates/core-shared"]` в корневом
  манифесте; `[package.metadata.leptos]` — только в корневом пакете (второе
  определение `[[workspace.metadata.leptos]]` сломало бы `cargo-leptos`).

## Соглашения

- **Definition of done**: билд-гейт после каждого логического этапа (fmt-check,
  check ssr, tests, clippy, build ssr; wasm-гейт перед коммитом) — см.
  [AGENTS.md](../../AGENTS.md) → «Definition of done». Полная связка
  cargo-leptos проверяется в CI, локально не требуется.
- **`just` локально не ставится** (машину не загромождаем посторонним ПО):
  justfile — декларация задач, прогон — в CI; локально используем прямые
  cargo-команды из раздела «Качество» в [development.md](../development.md).
- Коммиты — conventional commits (`feat:`, `fix:`, `docs:`, ...), формат версии —
  `vX.Y.Z-<hash>`; `CHANGELOG.md` генерируется git-cliff, руками не правится.
- Стилизация — только Tailwind-утилиты в `view!`-макросах; JS-конфиг Tailwind не
  заводить (v4, CSS-first).
- Стиль кода — `cargo fmt` + `cargo clippy -D warnings`, тесты рядом с модулями.
- Документация обновляется в том же изменении, что и код (см. `AGENTS.md`).

## Команды проверки

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --features ssr
cargo test --workspace --features ssr
cargo clippy --workspace --features ssr --all-targets -- -D warnings
cargo build --features ssr
cargo check --features hydrate --lib --target wasm32-unknown-unknown
cargo doc --workspace --no-deps
# cargo audit выполняется в CI, если локально инструмент не установлен.
```

## Known pitfalls

- `<A>` в leptos_router 0.8: классы — только через `attr:class`.
- `path!` — для маршрутов; `StaticSegment` — в корне `leptos_router`.
- `spawn_local` из `leptos::task` (не в prelude); сигналы не `Send`.
- `thaw-ui` 0.4 несовместим с leptos 0.8.
- Tailwind v4: бинарник подтягивает cargo-leptos, глобальный CLI не нужен.
- Windows: `npm.cmd`/`npx.cmd`, sccache отключён (`.cargo/config.toml`).
- YAML 1.1-парсеры могут трактовать `on` как boolean; это свойство парсера,
  не PowerShell/`Set-Content`. Проверка YAML не заменяет проверку GitHub Actions.
- **Reusable workflows**: вызывающая джоба должна разрешать необходимые
  `permissions`: дочерний workflow не может повысить права `GITHUB_TOKEN`.
  Workflow-level `env` родителя не наследуется.

## Решения, которые нельзя отменять без review

- Отказ от `thaw-ui` (см. `docs/stack.md`) до выхода версии под leptos 0.8.
- Отключение sccache в `.cargo/config.toml` (обход падения на Windows/web-sys).
- E2E в CI — только chromium (скорость); расширение — осознанное решение.
- Отсутствие предсжатия `.gz`/`.br` в сборке (сжатие — на внешнем слое) —
  см. `docs/asset-pipeline.md`.

## Ссылки

- [AGENTS.md](../../AGENTS.md) — карта проекта.
- [docs/current-state.md](../current-state.md) — состояние и риски.
- [docs/roadmap.md](../roadmap.md) — статусы фаз.
- [docs/architecture.md](../architecture.md), [docs/versioning.md](../versioning.md).