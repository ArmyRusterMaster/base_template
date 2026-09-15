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
| Общие DTO | `src/dto.rs` (план: `crates/core-shared` в воркспейсе) |
| Тесты | юнит — в модулях (`#[cfg(test)]`); e2e — `end2end/` |
| CI | `.github/workflows/ci.yaml` |
| Задачи разработчика | `justfile` |
| Релизы/CHANGELOG | `cliff.toml`, [docs/versioning.md](docs/versioning.md) |

## Команды

```bash
just --list        # список рецептов
just watch         # dev-сервер (127.0.0.1:3000)
just precommit     # fmt + clippy + tests + rustdoc
just test          # юнит-тесты + wasm-гейт
just e2e           # Playwright (нужен запущенный сервер)
```

Эквивалент вручную (и то, что гоняет CI):

```bash
cargo fmt --all
cargo clippy --features ssr --all-targets -- -D warnings
cargo test --features ssr
cargo check --features hydrate --lib --target wasm32-unknown-unknown
cargo audit
```

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