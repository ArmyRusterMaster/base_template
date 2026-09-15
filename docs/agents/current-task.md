# Current Task

## Goal

Восстановить компилируемость шаблона под leptos 0.8 / leptos_router 0.8 и
закрыть фазы 1, 5, 8, 9, 10, 11 [roadmap.md](../roadmap.md): `justfile` +
setup-скрипт, Tailwind вместо SCSS, CI-оптимизации, версионирование и авто-релиз,
устойчивые e2e-тесты, актуализация документации.

## Scope

- `src/app.rs`, `src/components.rs` — импорты и использование leptos_router 0.8.
- `Cargo.toml` — удаление несовместимых/неиспользуемых зависимостей, Tailwind.
- `style/main.css` (новый), удаление `style/main.scss`.
- `end2end/` — конфиг, константы, спеки (главная, навигация, 404, `/api/health`).
- `.github/workflows/ci.yaml` — wasm-гейт, binstall, джобы `audit` и `release`.
- `cliff.toml`, `justfile`, `scripts/setup.sh`, `deploy/Dockerfile`.
- Документация: `README.md`, `AGENTS.md`, `RULES.md`, `docs/*`, `docs/agents/*`.

## Out of scope

- Воркспейс `crates/core-shared` (фаза 7) — отдельная задача.
- Бизнес-слой (auth, оплата), CD (публикация образов), миграция `serde_yaml`.

## Current status

`completed` (с одной непроверенной локально частью — компиляция Rust).

## Completed

- Исправлены 7 ошибок компиляции из `_ck_err.log`: импорты (`A` из
  `leptos_router::components`, `path!` вместо `StaticSegment`), `spawn` →
  `leptos::task::spawn_local`, `class` → `attr:class` у `<A>`.
- Удалены `thaw` (leptos 0.7 — несовместим) и неиспользуемый `leptos-fetch`.
- Tailwind v4: `style/main.css`, `tailwind-input-file` в `Cargo.toml`, SCSS удалён.
- `justfile` + обновлённый `scripts/setup.sh` (cargo-binstall, just, git-cliff).
- CI: wasm-гейт, cargo-binstall, ожидание `/api/health`, e2e chromium + `BASE_URL`,
  smoke по `/` и `/api/health`, джобы `audit` и `release`.
- `cliff.toml`, `docs/versioning.md`, `AGENTS.md`, `docs/agents/*`,
  актуализированы все документы `docs/`.

## In progress

- Ничего (задача закрыта в рамках сессии).

## Next steps

1. На машине с Rust-тулчейном: `cargo fmt --all`, `cargo clippy ...`, `cargo test --features ssr`,
   `cargo check --features hydrate --target wasm32-unknown-unknown`, `just build`.
2. Пересобрать и закоммитить `Cargo.lock` после удаления зависимостей.
3. Фаза 7: воркспейс `crates/core-shared`.

## Blockers

- Локально нет доступного Rust-toolchain (`cargo`/`rustup` вне PATH, каталоги
  другого пользователя Windows) → компиляция не проверена.

## Files touched

`src/app.rs`, `src/components.rs`, `Cargo.toml`, `style/main.css` (new),
`style/main.scss` (deleted), `end2end/*`, `.github/workflows/ci.yaml`,
`cliff.toml` (new), `justfile` (new), `scripts/setup.sh`, `deploy/Dockerfile`,
`README.md`, `AGENTS.md` (new), `RULES.md`, `docs/*`, `docs/agents/*`.

## Checks run

- `end2end`: `npm install` + `npx tsc --noEmit` → exit 0.
- Валидация `.github/workflows/ci.yaml` (YAML, 9 джоб), `cliff.toml` и `Cargo.toml`
  (TOML) — выполнялась временным внешним скриптом вне репозитория, после проверки
  удалён (инструментальный валидатор конфигов будет вынесен в отдельный
  репозиторий и в этот проект не добавляется).
- `cargo *` не запускался (нет toolchain).

## Decisions made

- `thaw-ui` не используется: 0.4.x требует leptos 0.7 (см. `docs/stack.md`).
- Tailwind CLI не устанавливается глобально: бинарник скачивает cargo-leptos.
- E2E в CI — только chromium (быстрее); firefox/webkit — локально.
- Версия Tailwind не пинится в `Cargo.toml`: используется дефолт cargo-leptos
  (v4.2.1 на момент проверки, 15.09.2026) через `LEPTOS_TAILWIND_VERSION`.

## Risks

- Rust-код не компилировался локально: правильность проверена чтением кода против
  API leptos 0.8.20 / leptos_router 0.8.15 (docs.rs) и YAML/TOML/TS-валидацией.
- `Cargo.lock` устарел относительно `Cargo.toml` (cargo поправит при первой сборке).

## Last updated

15.09.2026