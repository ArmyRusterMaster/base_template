# Current Task

## Goal

План [2026-09-16-build-pipeline-and-tech-debt.md](../plans/2026-09-16-build-pipeline-and-tech-debt.md):
полная связка билда, техдолги, чистка локальной cargo-конфигурации, CD в GHCR.

## Scope

- `.cargo/config.toml`, `src/config.rs` (embedded-конфиг, `serde_yaml_ng`),
  `src/app.rs` (timeout/retry), `Cargo.toml`.
- CI: разбиение на reusable workflows, verify-артефакты, GHCR publish.
- `scripts/setup.sh`, docs/*.

## Out of scope

- Бизнес-слой, devcontainer, правка глобального `~/.cargo/config.toml`.
- Prerelease-теги в CD.

## Current status

`in_progress` — общий план техдолгов остаётся открытым.
Задача mimalloc/lld реализована, но сборка с этими изменениями ещё не
подтверждена CI. Прежние зелёные локальные гейты относятся к коду до аллокатора.

## Completed

- `.cargo/config.toml`: убраны `target-cpu=native` и `-fuse-ld=lld` (LNK4044 исчез).
- Definition of done (билд после каждого логического этапа) — AGENTS.md, working-memory.
- `just` выведен из локального процесса (setup.sh, development.md, README).
- Verify build artifacts в `build-release.yaml` (CSS/WASM/musl).
- Embedded-конфиг: `include_str!` + merge слоёв (embedded → файл → env) + 4 теста.
- Миграция `serde_yaml` → `serde_yaml_ng` 0.10 (cargo check/test/clippy зелёные).
- CI разбит: `ci.yaml` — оркестратор, 10 reusable-файлов `<job>.yaml`.
- GHCR: `publish.yaml` по стабильному тегу, теги `vX.Y.Z-<sha12>`, `vX.Y.Z`,
  `latest`; release ждёт publish.

## In progress

- E2E (2026-09-17): установка браузера Playwright 1.44.1 падает на Ubuntu
  24.04 из-за libasound2. Только e2e runner переведён на ubuntu-22.04;
  исправление ожидает CI. Локально браузеры/тесты/workflow не запускались.
  Следующий шаг: повтор CI, затем отдельное обновление Playwright/Chromium.

- Приоритет: исправление падения WASM release из лога CI владельца
  (2026-09-17): `queries overflow the depth limit` в `hydrate_async`.
  В `src/lib.rs` добавлен `recursion_limit = "256"`; из `Cargo.toml` убран
  игнорируемый `metadata.leptos.env`. SSR в присланном логе собрался успешно.
  Исправление ожидает повторного CI; локальные сборки не запускались.

- Timeout/retry `/api/health` (gloo-timers) и chef для WASM — по плану следующие.

## Next steps

1. Пуш в main → первый прогон нового CI; затем тестовый релизный тег для GHCR.
2. Этапы 5.5 (timeout/retry) и 5.4 (chef WASM).
3. Подтвердить в GitHub Actions сборку mimalloc/lld из `temp.md`.
   Реализация: native SSR allocator в `src/main.rs`, musl-only lld в CI/Docker;
   решение — [ADR-002](../decisions/002-mimalloc-lld.md).
   Локальные сборки, тесты и проверки workflow для этой задачи не запускать
   по указанию владельца.

## Blockers

Нет. `cargo audit`/`cargo deny` локально не установлены — проверяются в CI.

## Files touched

`.cargo/config.toml`, `.github/workflows/*` (9 новых + оркестратор), `Cargo.toml`,
`Cargo.lock`, `src/config.rs`, `scripts/setup.sh`, `docs/*`, `AGENTS.md`, `README.md`.

## Checks run (16.09.2026, локальный toolchain)

- Гейт: fmt-check, check ssr (workspace), test ssr (13 passed), clippy -D warnings,
  build ssr, wasm-гейт, doc, doctest — все exit 0 (`target/gate-results.txt`).
- `cargo audit`/`cargo deny` — команды отсутствуют на машине (exit 101), см. CI.
- `end2end`: tsc exit 0 (ранее в сессии); playwright --list exit 0 (24 теста).

## Risks

- Оркестратор с reusable workflows и GHCR-публикация не подтверждены удалённым
  прогоном (условия/права проверены статически).
- `cargo-leptos build` (полная связка) — только в CI.

## Last updated

16.09.2026