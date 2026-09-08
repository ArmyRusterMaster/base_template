# base_template

Базовый фуллстек-шаблон на **Rust + Leptos + Axum**: SSR-рендеринг с гидратацией,
типизированная конфигурация (YAML + env), Docker/Compose, GitHub Actions CI и
e2e-тесты на Playwright.

## Возможности

- **SSR + hydration**: страницы рендерятся на сервере, клиент принимает WASM.
- **Graceful shutdown** (Ctrl+C / SIGTERM).
- **Конфигурация** через `config/app.yaml` + переменные `APP_*` с валидацией
  (сервер не стартует при невалидных значениях).
- **Docker** мультистейдж: Ubuntu (build, cargo-chef + cargo-leptos) → Alpine (run).
- **Asset pipeline**: cache-busting (хэши в именах бандлов) из коробки.
- **CI/CD** (`.github/workflows/ci.yaml`): fmt, clippy, unit/e2e-тесты, Docker.
- **Playwright e2e** (`end2end/`).

## Быстрый старт

```bash
# локальная разработка (требует установки из scripts/setup.sh)
cargo leptos watch

# релизная сборка
cargo leptos build --release

# docker-compose
docker compose -f deploy/docker-compose.yaml up --build
```

## Документация

| Раздел | О чём |
|---|---|
| [docs/architecture.md](docs/architecture.md) | Структура проекта, потоки SSR/hydration |
| [docs/stack.md](docs/stack.md) | Стек технологий и инструменты |
| [docs/configuration.md](docs/configuration.md) | Конфигурация (YAML + env, валидация) |
| [docs/development.md](docs/development.md) | Окружение, команды, тесты |
| [docs/deployment.md](docs/deployment.md) | Docker, Compose, CI/CD |
| [docs/asset-pipeline.md](docs/asset-pipeline.md) | Сборка ассетов, cache-busting |
| [docs/best-practices.md](docs/best-practices.md) | Паттерны rust: newtype, builder, typestate |
| [docs/current-state.md](docs/current-state.md) | Текущее состояние проекта |
| [docs/roadmap.md](docs/roadmap.md) | План развития |