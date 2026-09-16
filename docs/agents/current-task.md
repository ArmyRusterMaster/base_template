# Current Task

## Goal

Фаза 7 [roadmap.md](../roadmap.md): вынести общие DTO из приложения в крейт
воркспейса `crates/core-shared`, чтобы типы не дублировались между SSR-сервером и
WASM-клиентом (RULES.md §5), и подготовить почву для доменных типов наследников.

## Scope

- `Cargo.toml` — `[workspace] members`, `resolver`, path-зависимость `core-shared`.
- `crates/core-shared/**` — крейт с DTO и тестами JSON-контракта.
- `src/lib.rs`, `src/app.rs`, `src/main.rs`, удаление `src/dto.rs`.
- CI-джобы `test`/`clippy`, `justfile` (`lint`, `test`) и `.dockerignore`.
- Документация: architecture, stack, development, roadmap, current-state, AGENTS,
  `docs/agents/*`.

## Out of scope

- `AppInfo` и прочие «типы на будущее» — в шаблоне нет потребителя.
- Вынос серверных модулей (`connectors`, `error`, `logging`) в отдельные крейты.
- Бизнес-слой, CD, миграция `serde_yaml`, timeout/retry клиентских запросов.

## Current status

`completed` — компиляция и тесты подтверждены локально (toolchain-прогон 16.09.2026).

## Completed

- Воркспейс: `[workspace] members = ["crates/core-shared"]`, `resolver = "2"`;
  `[package.metadata.leptos]` оставлен только в корневом пакете.
- Крейт `core-shared` (только `serde`): `HealthResponse`, `ConnectorHealthDto`;
  3 юнит-теста — JSON-контракт, round-trip, отсутствие `connectors`.
- `src/dto.rs` удалён; `src/app.rs` и `src/main.rs` используют `core_shared::*`.
- CI: шаги `cargo test -p core-shared` и `cargo clippy -p core-shared --all-targets`;
  justfile: `lint`/`test` покрывают оба пакета.
- `.dockerignore`: исключён `.kilo` (вложенные worktrees не идут в контекст сборки).
- Docs: раздел «Воркспейс» в architecture.md, обновлены stack/development/roadmap/
  current-state/AGENTS/working-memory.

## In progress

Ничего.

## Next steps

1. `just build` (cargo-leptos + Tailwind + wasm-pack) — локально или в CI
   (джоба `build-release`); `cargo check/test` уже зелёные.
2. Дальше по [roadmap.md](../roadmap.md): compile-time конфиг, CD (публикация
   образа), тег образа с версией, миграция `serde_yaml`.

## Blockers

Нет (таргет `wasm32-unknown-unknown` установлен на машину разработки).

## Files touched

`Cargo.toml`, `Cargo.lock`, `crates/core-shared/Cargo.toml`,
`crates/core-shared/src/lib.rs`, `src/lib.rs`, `src/app.rs`, `src/main.rs`,
`src/dto.rs` (удалён), `.github/workflows/ci.yaml`, `justfile`,
`.dockerignore`, `AGENTS.md`, `docs/architecture.md`, `docs/stack.md`,
`docs/development.md`, `docs/roadmap.md`, `docs/current-state.md`, `docs/agents/*`.

## Checks run (16.09.2026, локальный toolchain)

- `cargo fmt --all -- --check` → exit 0 (после `cargo fmt --all`).
- `cargo check --features ssr` → exit 0.
- `cargo clippy --features ssr --all-targets -- -D warnings` → ok.
- `cargo test --features ssr` → 10 passed / 0 failed (включая тесты core-shared).
- `cargo test -p core-shared` → 3 passed / 0 failed.
- `cargo check --features hydrate --lib --target wasm32-unknown-unknown` → ok
  (потребовался `rustup target add wasm32-unknown-unknown`).
- `cargo doc --workspace --no-deps` → ok.
- `end2end`: `npm install` + `npx tsc --noEmit` → exit 0.

## Decisions made

- `AppInfo` не создавали: тип без потребителя (добавляется наследниками).
- `[[workspace.metadata.leptos]]` не добавляли — cargo-leptos должен видеть ровно
  один проект (иначе потребуется `--project`).
- В `core-shared` запрещены `leptos`/`axum`/`tokio`: крейт обязан собираться под
  `wasm32-unknown-unknown`.
- `resolver = "2"` указан явно, чтобы резолв фич не менялся неявно.

## Risks

- Совместимость `cargo-leptos` с воркспейсом подтверждена на уровне
  `cargo metadata`/check; полный `cargo leptos build` — только в CI.

## Last updated

16.09.2026