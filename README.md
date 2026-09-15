# base_template

Базовый фуллстек-шаблон на **Rust + Leptos + Axum**: SSR-рендеринг с гидратацией,
типизированная конфигурация (YAML + env), Tailwind CSS v4, каркас коннекторов,
Docker/Compose, GitHub Actions CI (включая аудит зависимостей и авто-релизы) и
e2e-тесты на Playwright.

## Возможности

- **SSR + hydration**: страницы рендерятся на сервере, клиент принимает WASM-бандл.
- **Tailwind CSS v4** (CSS-first, `style/main.css`) — собирается cargo-leptos,
  standalone-бинарник скачивается автоматически.
- **UI-скелет**: layout (шапка с версией, адаптивное сайд-меню, футер),
  страницы `/`, `/account`, `/status`, 404 внутри layout.
- **Коннекторы**: трейт `Connector` + `ConnectorRegistry`; `GET /api/health`
  агрегирует статусы, версия приложения — там же.
- **Observability**: `tracing` (`RUST_LOG` → `log_level` → `info`),
  `x-request-id` на каждом запросе.
- **Ошибки**: единый `AppError` с безопасным JSON-ответом (без внутренних деталей).
- **Graceful shutdown** (Ctrl+C / SIGTERM).
- **Конфигурация** через `config/app.yaml` + переменные `APP_*` с валидацией:
  сервер не стартует при невалидных значениях (fail-fast).
- **Версионирование `vX.Y.Z-<hash>`** (в UI, `/api/health` и логе старта) и
  **авто-CHANGELOG** через git-cliff — см. [docs/versioning.md](docs/versioning.md).
- **Docker** мультистейдж: Ubuntu (build, cargo-chef + cargo-binstall) → Alpine (run).
- **Asset pipeline**: cache-busting (хэши в именах бандлов) из коробки.
- **CI/CD** (`.github/workflows/ci.yaml`): fmt (автокоммит) ∥ clippy ∥ audit →
  tests (+ wasm-гейт) → build-release → e2e ∥ docker → smoke → релиз по тегу.
- **justfile**: `just watch/build/serve/lint/test/doc/audit/e2e/docker/precommit`.
- **Playwright e2e** (`end2end/`): SSR-разметка, навигация, 404, `/api/health`.

## Быстрый старт

```bash
bash scripts/setup.sh      # окружение (Linux/macOS): Docker, Node, Rust, инструменты
just watch                 # dev-сервер: http://127.0.0.1:3000
just build                 # релизная сборка (сервер + WASM + CSS)
just precommit             # fmt + clippy + tests + rustdoc
just docker                # docker compose up --build
```

> Windows: вместо `npm`/`npx` используйте `npm.cmd`/`npx.cmd` (политики выполнения
> могут блокировать `*.ps1`) — подробности в [docs/development.md](docs/development.md).

## Документация

| Раздел | О чём |
|---|---|
| [docs/architecture.md](docs/architecture.md) | Структура проекта, модули, потоки SSR/hydration |
| [docs/stack.md](docs/stack.md) | Стек технологий и инструменты |
| [docs/configuration.md](docs/configuration.md) | Конфигурация (YAML + env, валидация) |
| [docs/development.md](docs/development.md) | Окружение, рецепты `just`, тесты |
| [docs/deployment.md](docs/deployment.md) | Docker, Compose, CI/CD |
| [docs/asset-pipeline.md](docs/asset-pipeline.md) | Tailwind, сборка ассетов, cache-busting |
| [docs/versioning.md](docs/versioning.md) | Версии, теги, CHANGELOG, релизы |
| [docs/best-practices.md](docs/best-practices.md) | Паттерны Rust: newtype, builder, typestate |
| [docs/current-state.md](docs/current-state.md) | Текущее состояние проекта |
| [docs/roadmap.md](docs/roadmap.md) | План развития |
| [docs/agents/](docs/agents/) | Заметки AI-агентов: задача, changelog, память |