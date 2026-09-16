# ADR-002: mimalloc для SSR-бинарника и lld для musl-линковки в CI

- Status: accepted
- Date: 2026-09-16
- Owners: template owner
- Related code: `Cargo.toml`, `src/main.rs`,
  `.github/workflows/build-release.yaml`, `deploy/Dockerfile`
- Related plan: [build-pipeline-and-tech-debt](../plans/2026-09-16-build-pipeline-and-tech-debt.md)

## Context

Задача из `temp.md`: «в CI добавить lld и оптимизированный аллокатор вместо
стандартного + отразить в CI и документации». При этом проектным решением
(см. `.cargo/config.toml`) machine-specific флаги сборки в шаблон не включаются.
Выигрыш аллокатора/линковщика на этом шаблоне не измерялся.

## Decision

- **Аллокатор**: `mimalloc 0.1.52` (`default-features = false`, optional)
  только для нативных платформ (`[target.'cfg(not(target_family = "wasm"))'.dependencies]`),
  включается фичей `ssr`; `#[global_allocator]` объявлен **только в SSR-бинарнике**
  (`src/main.rs`, `cfg(all(feature = "ssr", not(target_family = "wasm")))`).
  Библиотека (`lib`) и WASM-клиент аллокатор не переопределяют.
- **lld**: только для линковки `x86_64-unknown-linux-musl` в CI
  (`CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUSTFLAGS`) и Docker (ENV в
  base-стейдже, наследуется chef/build). Target-specific RUSTFLAGS не попадают
  в WASM, host-таргет и build scripts.
- Проектный `.cargo/config.toml` остаётся без флагов линкера/CPU — это решение
  машины разработчика, вне репозитория.

## Alternatives considered

- Системный аллокатор: не добавляет зависимостей, остаётся вариантом отката.
- jemalloc и snmalloc: альтернативы не интегрировались и не сравнивались
  на нагрузке проекта; сравнительные выводы о скорости и размере не делаются.
- Общие флаги lld для всех платформ: не выбраны; Linux-флаги нельзя передавать
  MSVC или WASM. Локальная конфигурация остаётся без них.

## Consequences

- `libmimalloc-sys` компилирует C через `cc`: в CI/Docker уже есть
  `build-essential`/`musl-gcc`; локально MSVC — из VS Build Tools.
- Аллокатор меняет поведение всего SSR-процесса; откат — убрать `dep:mimalloc`
  из фичи `ssr` и static из `src/main.rs` (две точки).
- Выигрыш по скорости/памяти **не измерялся** и не заявляется (см. ниже).

## Security impact

- Добавлена native-зависимость `libmimalloc-sys` с C-кодом и его build script.
  Это расширяет supply-chain и область небезопасного кода; `cargo audit` в CI
  проверяет известные advisory, но не доказывает безопасность исходников C.
- lld меняет инструмент сборки; HTTP-контракт и проверки доступа не меняются.

## Performance impact

- Улучшения не измерялись (нет baseline/profile) — решение принято как
  «дешёвый дефолт, который можно откатить», а не как оптимизация.
  Любые заявления об ускорении — только после прогонов по правилам
  профилирования (RULES.md §8).

## Migration and rollback

- Откат аллокатора: удалить `"dep:mimalloc"` из `ssr` и `#[global_allocator]`.
- Откат lld: удалить одну env-строку в CI и одну ENV-строку в Dockerfile.

## Verification

- Манифест и lockfile проверены локально парсингом (`cargo update --workspace`,
  exit 0, без компиляции). Компиляция/тесты/линковка — CI: `test` (юнит-тесты,
  wasm-гейт) и `build-release` (musl + lld + верификация артефактов).
  Локальные билды по решению владельца не запускаются.
