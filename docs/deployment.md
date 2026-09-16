# Деплой: Docker, Compose, CI/CD

## Dockerfile (`deploy/Dockerfile`)

Мультистейдж-сборка:

```
base    (Ubuntu 24.04)  rustup + wasm/musl targets + cargo-chef + cargo-leptos
planner                  cargo chef prepare  → recipe.json
deps                     cargo chef cook (release, ssr, x86_64-unknown-linux-musl)
build    (Ubuntu)        cargo-leptos build --release
run      (Alpine 3.20)   статический бинарник + site + config, непривилегированный юзер
```

Ключевые решения:

- **Конфигурация сборки — из проекта**: `[package.metadata.leptos]` в Cargo.toml
  (site-root, `tailwind-input-file`, assets-dir, фичи bin/lib) и `config/app.yaml`.
- **cargo-binstall**: `cargo-chef` и `cargo-leptos` ставятся готовыми бинарниками
  (fallback — `cargo install`), это экономит десятки минут на холодном билде.
- **Tailwind**: standalone-бинарник скачивает cargo-leptos на этапе сборки —
  в образе рантайма ничего дополнительно не нужно.
- **Сервер собирается под musl** (`LEPTOS_BIN_TARGET_TRIPLE=x86_64-unknown-linux-musl`,
  линковщик musl-gcc, компоновка через lld) → полностью статический бинарник,
  который запускается на Alpine без glibc.
- **cargo-chef** кеширует зависимости в отдельном слое — повторные билды быстрые.
- В рантайме ENV: `LEPTOS_SITE_ROOT=/app/site`, `LEPTOS_SITE_PKG_DIR=pkg`,
  `APP_SERVER_HOST=0.0.0.0`, `APP_SERVER_PORT=3000`.

## Dockerfile.prebuilt (`deploy/Dockerfile.prebuilt`)

Упрощённый рантайм-образ для CI: собирается из **готового артефакта**
(бинарник + `site/` + `config/`), без пересборки Rust в контейнере.
Используется джобой `docker` в CI — бинарь собирается один раз в `build-release`
и раскладывается по остальным стейджам.

## Docker Compose (`deploy/docker-compose.yaml`)

```bash
docker compose -f deploy/docker-compose.yaml up --build
# приложение: http://localhost:3000
```

Сервис `app` содержит проброс порта 3000, `restart: unless-stopped` и healthcheck
(`wget` на `/`). Конечный образ: `base-template:latest`.

## CI/CD (`.github/workflows/ci.yaml`)

E2E временно закреплён на `ubuntu-22.04`: Playwright 1.44.1 из
`end2end/package-lock.json` использует зависимости Jammy (`libasound2`),
несовместимые с установкой на Noble (Ubuntu 24.04, `libasound2t64`).
`npm ci` и `playwright install --with-deps chromium` сохраняются.
Перед переходом e2e на Ubuntu 24.04 нужно обновить Playwright вместе с браузером.
Это мера совместимости, не обновление безопасности устаревшего браузера;
исправление ожидает повторного CI.

Конвейер (порядок + параллельность):

`ci.yaml` — только оркестратор (`needs`, условия, права). Каждая джоба вынесена
в одноимённый `.github/workflows/<job>.yaml` с `workflow_call`. Они вызываются
из того же коммита и обмениваются артефактами одного запуска. Workflow-level
`env` родителя не наследуется — нужные переменные заданы в дочерних файлах.
Права write разрешены также на вызывающих jobs, поскольку дочерний workflow
не может повысить права токена. GITHUB_TOKEN доступен без `secrets: inherit`.

```
1. fmt ∥ clippy ∥ audit — параллельно
2. fmt автокоммит      — push в main коммитит отформатированный код ([skip ci]);
                         на PR — ошибка, если код не отформатирован
3. test (+ wasm gate)  — юнит-тесты и сборка клиента под wasm32 (после fmt+clippy)
4. build-release       — musl-статик бинарник + WASM + CSS → артефакт `release`
5. e2e ∥ docker        — параллельно, оба из артефакта:
                         e2e    — Playwright (chromium) против бинарника;
                         docker — образ из готового бинарника (Dockerfile.prebuilt)
6. smoke               — отдельный стейдж: запуск контейнера + HTTP-проверки
7. publish             — по стабильному тегу: GHCR, ждёт smoke + e2e + audit
8. release             — после publish: CHANGELOG.md (git-cliff) + GitHub Release
```

| Джоба | Что делает |
|---|---|
| `fmt` | `cargo fmt --all`; на push в main — автокоммит `style: cargo fmt [skip ci]` |
| `clippy` | `cargo clippy --features ssr --all-targets -- -D warnings` |
| `audit` | `cargo audit` (RustSec Advisory DB), независимая джоба |
| `test` | `cargo test --features ssr --lib` + `cargo check --features hydrate --lib --target wasm32-unknown-unknown` |
| `build-release` | Установка `musl-tools` + `lld`; `cargo-leptos build --release` (musl-сервер, линковка lld, mimalloc в бинарнике) + upload артефакта `dist/` |
| `e2e` | Скачивает артефакт, ждёт готовности `/api/health`, гоняет Playwright (`--project=chromium`, `BASE_URL`) |
| `docker` | Собирает образ из артефакта (`deploy/Dockerfile.prebuilt`), сохраняет в артефакт |
| `smoke` | Загружает образ, запускает контейнер, проверяет `/` и `/api/health` |
| `release` | По тегу `v*`: генерирует `CHANGELOG.md` (git-cliff) и создаёт GitHub Release |

Ожидание готовности сервисов реализовано циклом по `/api/health` (вместо
фиксированного `sleep`), поэтому джобы устойчивы к разной скорости старта.

Версионирование, теги и генерация CHANGELOG — [versioning.md](versioning.md).

## Публикация в GHCR

Джоба `publish` в `.github/workflows/ci.yaml` реализована; первый удалённый
прогон ещё не подтверждён. Запускается только на push тега стабильной версии
`vX.Y.Z`, после успешных `smoke`, `e2e` и `audit`. GitHub Release ждёт `publish`.
PR не публикуют образы. Пререлизные теги пока не поддерживаются.

Публикуется **тот же** `base-template:ci` из артефакта `docker-image`, который
прошёл smoke, без пересборки. Адрес — `ghcr.io/<owner>/<repository>` в нижнем
регистре; теги — `vX.Y.Z-<12 символов commit SHA>`, `vX.Y.Z`, `latest`.
`latest` означает последний успешно опубликованный релиз, а не обязательно
максимальную версию: повторный запуск старого релиза может переместить тег.
Для воспроизводимого развёртывания используйте digest образа, а не `latest`.

Авторизация — стандартный `GITHUB_TOKEN` через `docker login --password-stdin`.
`packages: write` есть только у `publish`; `contents: write` — у существующих
джоб `fmt` и `release`. Остальные джобы имеют `contents: read`.
Организация должна разрешать Actions публиковать packages; для уже существующего
пакета нужно предоставить этому репозиторию доступ. Видимость пакета настраивается
в GHCR отдельно. PAT и установка инструментов локально не требуются.

Пуш трёх тегов не атомарен: при частичном отказе возможна публикация части тегов.
Повторный запуск использует тот же артефакт, пока он доступен (retention — 1 день).
Автоматический деплой на сервер не реализован.
