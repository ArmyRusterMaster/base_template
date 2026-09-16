# Change log для агентов

Краткие записи об изменениях, полезных будущим агентам. Формат:

```
## YYYY-MM-DD — Краткое название
- Changed:
- Why:
- Files:
- Tests:
- Docs:
- Risks:
```

## 16.09.2026 — Фаза 7: воркспейс `crates/core-shared`

- **Changed:** корневой манифест стал воркспейсом
  (`[workspace] members = ["crates/core-shared"]`, `resolver = "2"`); создан крейт
  `crates/core-shared` (только `serde`): `HealthResponse`, `ConnectorHealthDto` +
  тесты JSON-контракта; `src/dto.rs` удалён, приложение использует
  `core_shared::*`; в CI/just добавлены `cargo test -p core-shared` и
  `cargo clippy -p core-shared`; в `.dockerignore` добавлен `.kilo` (вложенные
  worktrees не должны попадать в контекст сборки).
- **Why:** общие типы не должны дублироваться между SSR и WASM (RULES.md §5);
  крейт без leptos/axum/tokio гарантирует сборку под wasm32.
- **Files:** `Cargo.toml`, `crates/core-shared/**` (new), `src/lib.rs`,
  `src/app.rs`, `src/main.rs`, `src/dto.rs` (удалён),
  `.github/workflows/ci.yaml`, `justfile`, `.dockerignore`, `AGENTS.md`, `docs/**`.
- **Tests:** локальный toolchain-прогон 16.09.2026: `cargo fmt --all -- --check` ok,
  `cargo check --features ssr` ok, `cargo clippy --features ssr --all-targets --
  -D warnings` ok, `cargo test --features ssr` → 10 passed, `cargo test -p
  core-shared` → 3 passed, wasm-гейт `cargo check --features hydrate --lib
  --target wasm32-unknown-unknown` ok, `cargo doc --workspace --no-deps` ok.
  Для wasm-гейта потребовался `rustup target add wasm32-unknown-unknown`.
- **Docs:** architecture (+ раздел «Воркспейс»), stack, development, roadmap
  (Фаза 7 → done), current-state, AGENTS, docs/agents/*.
- **Risks:** совместимость `cargo-leptos` с воркспейсом подтверждается только в
  CI; `Cargo.lock` требует пересборки (добавлен path-крейт). `AppInfo` намеренно
  не создавали — нет потребителя.

## 16.09.2026 — Компилируемость + фазы 1, 5, 8, 9, 10, 11

- **Changed:** исправлены ошибки компиляции под leptos 0.8
  (`A` из `leptos_router::components`, `path!` вместо `StaticSegment`,
  `leptos::task::spawn_local` вместо `spawn`, `attr:class` вместо `class`);
  удалены `thaw` и `leptos-fetch`; Tailwind v4 вместо SCSS
  (`style/main.css`, `tailwind-input-file`); добавлены `justfile`, `cliff.toml`,
  `AGENTS.md`, `docs/agents/*`; CI: wasm-гейт, cargo-binstall, ожидание
  `/api/health`, e2e chromium + `BASE_URL`, smoke по `/` и `/api/health`, джобы
  `audit` и `release`; e2e-тесты переписаны (главная, навигация, 404, API).
- **Why:** прошлый `cargo check` падал с 7 ошибками; CI-проверки e2e/smoke
  ссылались на старый заголовок «Welcome to Leptos» и были заведомо красными;
  страницы использовали Tailwind-классы без Tailwind в сборке.
- **Files:** `src/app.rs`, `src/components.rs`, `Cargo.toml`, `style/main.css`,
  `style/main.scss` (удалён), `end2end/*`, `.github/workflows/ci.yaml`,
  `cliff.toml`, `justfile`, `scripts/setup.sh`, `deploy/Dockerfile`, `README.md`,
  `AGENTS.md`, `RULES.md`, `docs/*`.
- **Tests:** `end2end`: `npm install` + `npx tsc --noEmit` → exit 0; YAML/TOML
  валидация CI и конфигов. `cargo` не запускался (нет toolchain).
- **Docs:** добавлен `docs/versioning.md`; актуализированы architecture, stack,
  asset-pipeline, configuration, development, deployment, current-state, roadmap,
  README, docs/README.
- **Risks:** Rust-код не компилировался локально; `Cargo.lock` устарел после
  удаления зависимостей (обновится при первой сборке).

## 08.09.2026 — Первый `cargo check` и фиксация ошибок

- **Changed:** ничего в репозитории (только артефакты проверки `_ck_out.log` /
  `_ck_err.log`).
- **Why:** зафиксировать список ошибок компиляции перед следующей сессией.
- **Files:** временные логи `cargo check` (в репозитории не хранятся и не
  коммитятся; `*.log` в `.gitignore`).
- **Tests:** `cargo check` → 7 ошибок (импорты leptos_router, `spawn`, `class`
  у `<A>`).
- **Docs:** ошибки перенесены в `docs/current-state.md` → Known limitations.
- **Risks:** компиляция оставалась сломанной до сессии 16.09.2026.