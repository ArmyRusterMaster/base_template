# AGENTS.md — карта проекта для AI-агентов

Точка входа: что это за проект, где что лежит, какие правила и проверки
обязательны.

## Что это

`base_template` — базовый фуллстек-шаблон на **Rust + Leptos 0.8 (SSR +
hydration) + Axum 0.8**. Бизнес-логики нет: это каркас, из которого делаются
производные шаблоны (в т.ч. шаблон микросааса). Наследники добавляют auth,
оплату, конкретные коннекторы и свои домены.

## Обязательное чтение перед изменением

1. [docs/current-state.md](docs/current-state.md) — что уже реализовано,
   ограничения и риски.
2. [docs/architecture.md](docs/architecture.md) — модули, потоки, правила
   leptos/leptos_router 0.8.
3. [docs/roadmap.md](docs/roadmap.md) — статусы фаз и текущий техдолг.
4. [RULES.md](RULES.md) — правила кода, коммитов и версионирования.
5. [docs/agents/current-task.md](docs/agents/current-task.md) — активная задача.
6. [docs/agents/working-memory.md](docs/agents/working-memory.md) — устойчивые
   соглашения и известные ловушки.

## Карта

| Что | Где |
|---|---|
| Конфигурация сборки | `Cargo.toml` (`[package.metadata.leptos]`, features `ssr`/`hydrate`) |
| Конфигурация рантайма | `config/app.yaml` + `APP_*`; код — `src/config.rs` |
| SSR-сервер | `src/main.rs` (роуты, `/api/health`, request-id, graceful shutdown) |
| UI | `src/app.rs` (shell, маршруты, страницы), `src/components.rs` (layout) |
| Стили | `style/main.css` (Tailwind v4, CSS-first) |
| Контракт интеграций | `src/connectors/mod.rs` |
| Ошибки | `src/error.rs` |
| Логирование | `src/logging.rs` |
| Версия сборки | `build.rs` → `src/version.rs` |
| Общие DTO | `crates/core-shared` (крейт воркспейса) |
| Тесты | юнит — в модулях (`#[cfg(test)]`) и `crates/core-shared`; e2e — `end2end/` |
| CI | `.github/workflows/ci.yaml` |
| Задачи разработчика | `justfile` |
| Релизы/CHANGELOG | `cliff.toml`, [docs/versioning.md](docs/versioning.md) |

## Команды

`just` локально не ставится — его роль выполняет CI. Прямые команды:

```bash
cargo fmt --all -- --check          # = рецепт fmt-check
cargo clippy --features ssr --all-targets -- -D warnings   # = lint (+ core-shared)
cargo clippy -p core-shared --all-targets -- -D warnings
cargo test --features ssr           # = test
cargo test -p core-shared
cargo check --features hydrate --lib --target wasm32-unknown-unknown   # WASM-гейт
cargo build --features ssr          # = build
cargo audit                         # = audit
```

Full-связка (`cargo leptos build --release`) прогоняется в CI; локальный
dev-сервер — `cargo leptos watch`. Полный список рецептов — [justfile](justfile),
соответствие рецептам — [docs/development.md](docs/development.md).

Эквивалент вручную (и то, что гоняет CI):

```bash
cargo fmt --all
cargo clippy --features ssr --all-targets -- -D warnings
cargo clippy -p core-shared --all-targets -- -D warnings
cargo test --features ssr
cargo test -p core-shared
cargo check --features hydrate --lib --target wasm32-unknown-unknown
cargo audit
```

## Definition of done (билд после каждого логического этапа)

Любой логический этап работы (фича, рефакторинг, техдолг) считается
завершённым только после локального гейта:

```bash
cargo fmt --all -- --check
cargo check --features ssr
cargo test --features ssr        # включает cargo test -p core-shared
cargo clippy --features ssr --all-targets -- -D warnings
cargo build --features ssr       # линковка, а не только проверка типов
```

и wasm-гейта перед коммитом:

```bash
cargo check --features hydrate --lib --target wasm32-unknown-unknown
```

Полная связка (`cargo leptos build`: Tailwind + wasm-pack) локально **не**
требуется — её проверяет CI (джоба `build-release` с шагом верификации
артефактов). Записи о пройденных гейтах фиксируются в
[docs/agents/current-task.md](docs/agents/current-task.md) и
[docs/agents/change-log.md](docs/agents/change-log.md).

## Known pitfalls

- `<A>` (leptos_router 0.8) **не принимает `class`** — только attribute
  spreading: `<A href="/" attr:class="...">`.
- Пути маршрутов задаются макросом `path!`; `StaticSegment` — в корне
  `leptos_router` (не в `components`).
- `spawn`/`spawn_local` не в prelude: `use leptos::task::spawn_local;` (сигналы
  не `Send`).
- `thaw-ui` 0.4 требует leptos 0.7 — несовместим с 0.8, не добавлять.
- Tailwind v4: JS-конфиг не нужен (`@theme`/`@source`); версия standalone-бинарника
  — `LEPTOS_TAILWIND_VERSION`; глобальный tailwind CLI не ставить.
- Windows: `npm`/`npx` вызывать как `npm.cmd`/`npx.cmd`; sccache отключён
  (`.cargo/config.toml`, `rustc-wrapper = ""`).
- `CHANGELOG.md` — генерируемый файл, руками не править.
- Общие типы (клиент + сервер) живут **только** в `crates/core-shared`: DTO в
  `src/` дублировать нельзя, а крейт не должен зависеть от `leptos`/`axum`/`tokio`
  (иначе сломается сборка под `wasm32`).