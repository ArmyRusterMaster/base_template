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
- Сборка статики — через cargo-leptos (см. [asset-pipeline.md](asset-pipeline.md)).
- В рантайме ENV: `LEPTOS_SITE_ROOT=/app/site`, `LEPTOS_SITE_PKG_DIR=pkg`,
  `APP_SERVER_HOST=0.0.0.0`, `APP_SERVER_PORT=3000`.

## Docker Compose (`deploy/docker-compose.yaml`)

```bash
docker compose -f deploy/docker-compose.yaml up --build
# приложение: http://localhost:3000
```

Сервис `app` содержит проброс порта 3000, `restart: unless-stopped` и healthcheck
(`wget` на `/`). Конечный образ: `base-template:latest`.

## CI/CD (`.github/workflows/ci.yaml`)

Джобы:

| Джоба | Что проверяет |
|---|---|
| `check` | `cargo fmt`, `clippy -D warnings`, unit-тесты |
| `build-release` | полная сборка `cargo-leptos build --release` + smoke-тест сервера |
| `e2e` | Playwright (chromium) против релиз-сборки |
| `docker` | сборка образа и smoke-тест контейнера |

Публикация образов/деплой (CD) — план, см. [roadmap.md](roadmap.md).