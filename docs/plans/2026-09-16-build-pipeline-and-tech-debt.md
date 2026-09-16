# План: полная связка билда, техдолги, локальная конфигурация

- Status: in_progress
- Date: 2026-09-16
- Owners: template owner
- Related code: `.cargo/config.toml`, `src/config.rs`, `src/app.rs`,
  `deploy/Dockerfile`, `.github/workflows/ci.yaml`, `scripts/setup.sh`,
  `crates/core-shared`, `docs/*`
- Related plan: [roadmap.md](../roadmap.md)

## Goal

1. Проверить **полную связку билда** (cargo-leptos + Tailwind + wasm-pack) — в CI.
2. Зафиксировать правило процесса: **билд после каждого логического этапа**.
3. Выполнить **техдолги** из roadmap (compile-time конфиг, миграция `serde_yaml`,
   timeout/retry клиентских запросов, chef для WASM, CD: тег образа + GHCR).
4. Убрать из **проектной** конфигурации сборки machine-specific флаги
   (`target-cpu=native`, `-fuse-ld=lld`).
5. Вывести `just` из локального процесса: justfile — декларация задач, прогон — в CI.

## Non-goals

- Бизнес-слой (auth, оплата, конкретные коннекторы), devcontainer.
- Установка `just` / `cargo-leptos` на машину разработки.
- Изменение контракта `/api/health`.
- Правка глобального `~/.cargo/config.toml` пользователя (вне репозитория).

## Current state

- `cargo check/clippy/test/doc`, wasm-гейт, `cargo build --features ssr` — зелёные
  локально (16.09.2026); `cargo leptos build` не прогонялся.
- `Cargo.lock` актуален; в рантайме rustc получает `-C link-arg=-fuse-ld=lld -C
  target-cpu=native` из проектного `.cargo/config.toml` → `LNK4044` на MSVC.
- `serde_yaml` deprecated (advisory unmaintained); конфиг читается с диска.
- Клиентский запрос `/api/health` — без таймаута/retry.
- Docker-сборка кеширует только musl-зависимости; WASM-дерево компилируется
  каждый раз.

## Proposed design

- `.cargo/config.toml` — только `rustc-wrapper = ""`; флаги линкера/CPU — решение
  машины разработчика.
- Definition of done этапа: fmt-check → check ssr → test ssr + core-shared →
  clippy -D warnings → build ssr; wasm-гейт перед коммитом.
- Верификация артефактов в CI (`build-release`): наличие CSS (Tailwind отработал:
  `.rounded-xl` в минифицированном CSS), `.wasm`, musl-бинарника.
- Конфигурация: слои `embedded include_str!(../config/app.yaml)` →
  `config/app.yaml` (если есть) → `APP_*`; слияние YAML-документов на уровне
  `Value`.
- GHCR: джоба `publish` по тегу `v*` — образ из артефакта `docker`, теги
  `vX.Y.Z` + `latest`, логин через `GITHUB_TOKEN` (`packages: write`), owner
  приводится к нижнему регистру.
- Timeout/retry: 2 попытки, таймаут 5 c (gloo-timers), деградация UI вместо
  вечной заглушки.

## Affected components

`.cargo/config.toml`, `src/config.rs`, `src/app.rs`, `Cargo.toml` (`serde_yml`,
`gloo-timers`, `futures`), `deploy/Dockerfile`, `.github/workflows/ci.yaml`,
`scripts/setup.sh`, `docs/*`.

## Invariants

- Контракт `/api/health` не меняется (тест JSON в `core-shared`).
- Приоритет источников конфигурации: embedded → файл → env (env сильнейший).
- wasm-гейт зелёный после каждого этапа; `Cargo.lock` актуален.

## Security impact

- CD: пуш в GHCR только по тегу `v*`, права `packages: write` только у джобы
  `publish`, секреты — стандартный `GITHUB_TOKEN` (без PAT).
- Compile-time конфиг не должен вшивать секреты (`app.yaml` — не секрет).

## Performance impact

- Удаление `target-cpu=native`: потенциально медленнее локальный дебаг-бинарник —
  незначимо; единообразие важнее.
- chef для WASM: ускорение повторных docker-сборок.

## Migration

- `serde_yaml` → `serde_yml` (совместимый форк 0.9 API).
- `config/app.yaml` остаётся в репо и в Docker-образе; embedded — фолбэк.

## Steps

1. Этап 0: этот документ.
2. Этап 1: чистка `.cargo/config.toml` + билд-гейт.
3. Этап 2: definition of done в AGENTS.md/working-memory.
4. Этап 3: `just` — вне локального процесса (setup.sh, development.md, README).
5. Этап 4: Verify build artifacts в CI.
6. Этап 5.1: embedded-конфиг (merge YAML-слоёв) + тесты.
7. Этап 5.2: миграция на `serde_yml`.
8. Этап 5.5: timeout/retry `/api/health` + деградация UI.
9. Этап 5.4: chef cook для WASM в Dockerfile.
10. Этап 5.3: GHCR publish (CI) + docs/deployment.md.
11. Этап 6: закрытие плана, финальный гейт.

## Verification

- Локально после каждого шага: гейт из Proposed design.
- Полная связка: зелёная джоба `build-release` с шагом верификации артефактов.
- CD: пуш образа в GHCR по тегу (проверяется при первом релизе).

## Risks

- `serde_yml` — пре-1.0 форк; поведение `deny_unknown_fields` перепроверить
  тестами.
- chef для WASM при несовпадении target-dir просто не даст кеша (без поломок).
- GHCR: owner репозитория должен быть в нижнем регистре (приводим скриптом).

## Open questions

- Порядок техдолгов подтверждён владельцем; реестр — GHCR; `-fuse-ld=lld` для
  linux-gnu тоже убирается.

## Status

in_progress — этапы 0–5.3 (кроме 5.5/5.4) выполнены, локальные гейты зелёные.
Осталось: этап 5.5 (timeout/retry), 5.4 (chef WASM), первый удалённый прогон
CI и тестовый релизный тег для подтверждения GHCR.


## Дополнение: задача из temp.md (16.09.2026)

- Реализован mimalloc для native SSR-бинарника; WASM аллокатор не меняется.
- lld включён только для musl-линковки в CI и Docker, не в локальном Cargo config.
- Решение: [ADR-002](../decisions/002-mimalloc-lld.md).
- Lockfile обновлён; локальные сборки, тесты и проверки workflow для этой задачи
  не выполняются по указанию владельца. Подтверждение сборки ожидается в CI.
- Общий статус плана остаётся `in_progress`; timeout/retry и chef WASM не закрыты.

