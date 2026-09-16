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

## 2026-09-17 — Совместимость e2e runner с Playwright 1.44.1

- Changed: `.github/workflows/e2e.yaml` — Ubuntu 22.04 вместо ubuntu-latest.
- Why: установка браузера падала на Ubuntu 24.04 до тестов: у libasound2
  нет installation candidate. Версия из lockfile поддерживает Ubuntu 22.04.
- Tests: разобран лог CI и таблица nativeDeps Playwright v1.44.1;
  локальные тесты, установка браузера и проверки workflow не запускались.
- Docs: deployment, current-task.
- Risks: временное закрепление ОС; обновление Playwright/Chromium отложено.
  Требуется повторный CI, успешные e2e пока не подтверждены.


## 2026-09-17 — WASM release: query-depth overflow

- Changed: `src/lib.rs` — `recursion_limit = "256"` по рекомендации rustc;
  из `Cargo.toml` удалён неподдерживаемый `metadata.leptos.env`.
- Why: полный лог CI показывает overflow layout типов `hydrate_async` Leptos;
  серверная musl-сборка закончилась успешно, общий cargo-leptos — exit 1.
- Tests: разобран лог владельца и просмотрен diff. Локальные сборки/тесты
  и проверки workflow не запускались по указанию владельца. Нужен повтор CI.
- Risks: исправление ещё не подтверждено release-сборкой WASM;
  runtime/API и mimalloc/lld не менялись.


## 16.09.2026 — Аллокатор mimalloc + lld для musl (задача из temp.md)

- **Changed:** `Cargo.toml` — `mimalloc 0.1.52` (optional, `default-features =
  false`) в `[target.'cfg(not(target_family = "wasm"))'.dependencies]` + `dep:mimalloc`
  в фиче `ssr`; `src/main.rs` — `#[global_allocator]` только в SSR-бинарнике
  (`cfg(all(feature = "ssr", not(target_family = "wasm")))`); `build-release.yaml` —
  `lld` + `CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-C link-arg=-fuse-ld=lld"`;
  `deploy/Dockerfile` — `lld` в apt-слое и те же env в base-стейдже; `test.yaml` —
  `cargo test --features ssr` теперь включает тестовую сборку SSR-бинарника в CI.
- **Why:** задача 2 из `temp.md` («добавить lld и оптимизированный аллокатор в CI
  и документацию»).
- **Files:** `Cargo.toml`, `Cargo.lock`, `src/main.rs`,
  `.github/workflows/build-release.yaml`, `.github/workflows/test.yaml`,
  `deploy/Dockerfile`, `docs/decisions/002-mimalloc-lld.md` (новый), docs/*.
- **Tests:** компиляция/тесты локально **не запускались** (решение владельца);
  `cargo update --workspace` → exit 0 (lock: `mimalloc 0.1.52`,
  `libmimalloc-sys 0.1.49`). Сборку подтверждает CI (`test` + `build-release`).
- **Docs:** stack, deployment, current-state, roadmap, ADR-002.
- **Risks:** mimalloc компилирует C (`cc`) — в CI/Docker тулчейн есть, локальный
  MSVC должен иметь C-инструменты; выигрыш производительности не измерялся и не
  заявляется.


## 16.09.2026 — Техдолги, CI-оркестратор, GHCR

- **Changed:** `.cargo/config.toml` без `target-cpu=native`/`-fuse-ld=lld`;
  embedded-конфиг (`include_str!` + merge embedded→файл→env, `serde_yaml_ng`
  0.10); definition of done в AGENTS/working-memory; `just` из локального
  процесса убран; CI разбит на 10 reusable workflows + оркестратор `ci.yaml`;
  verify-артефактов в `build-release`; `publish.yaml` → GHCR по стабильному тегу
  (`vX.Y.Z-<sha12>`, `vX.Y.Z`, `latest`), release ждёт publish.
- **Why:** техдолги roadmap, машинно-специфичные флаги не место в шаблоне,
  GHCR выбран владельцем.
- **Files:** перечислены в `docs/agents/current-task.md`.
- **Tests:** локальный гейт — fmt/check/test(13)/clippy/build/wasm/doc/doctest
  все exit 0; `cargo audit`/`deny` не установлены локально (в CI есть).
- **Docs:** deployment, configuration, architecture, current-state, roadmap,
  development, README, AGENTS, agents/*.
- **Risks:** удалённый CI и GHCR не подтверждены; timeout/retry (5.5) и chef
  WASM (5.4) не начаты. Аллокатор и lld реализованы следующей записью выше;
  результат их сборки ожидается в CI.

## 16.09.2026 — Фаза 7: core-shared

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