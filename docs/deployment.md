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
  (site-root, style-file, assets-dir, фичи bin/lib) и `config/app.yaml`.
- **Сервер собирается под musl** (`LEPTOS_BIN_TARGET_TRIPLE=x86_64-unknown-linux-musl`,
  линковщик musl-gcc) → полностью статический бинарник, который запускается
  на Alpine без glibc.
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

Конвейер (порядок + параллельность):

```
1. fmt ∥ clippy        — параллельно
2. fmt автокоммит      — push в main коммитит отформатированный код ([skip ci]);
                         на PR — ошибка, если код не отформатирован
3. test                — юнит-тесты (после fmt+clippy)
4. build-release       — musl-статик бинарник + WASM + CSS → артефакт `release`
5. e2e ∥ docker        — параллельно, оба из артефакта:
                         e2e    — Playwright против бинарника;
                         docker — образ из готового бинарника (Dockerfile.prebuilt)
6. smoke               — отдельный стейдж: запуск контейнера + HTTP-проверка
```

| Джоба | Что делает |
|---|---|
| `fmt` | `cargo fmt --all`; на push в main — автокоммит `style: cargo fmt [skip ci]` |
| `clippy` | `cargo clippy --features ssr --all-targets -- -D warnings` |
| `test` | `cargo test --features ssr --lib` |
| `build-release` | `cargo-leptos build --release` (musl-сервер) + upload артефакта `dist/` |
| `e2e` | Скачивает артефакт, стартует сервер, гоняет Playwright |
| `docker` | Собирает образ из артефакта (`deploy/Dockerfile.prebuilt`), сохраняет в артефакт |
| `smoke` | Загружает образ, запускает контейнер, `curl` / grep заголовка |

Публикация образов/деплой (CD) — план, см. [roadmap.md](roadmap.md).