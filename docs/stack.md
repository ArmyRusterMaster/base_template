# Стек технологий

## Backend / SSR

| Технология | Назначение | Ресурсы |
|---|---|---|
| **axum 0.8** | HTTP-сервер, роутинг, serve | [axum.rs](https://github.com/tokio-rs/axum) |
| **leptos 0.8** | Реактивный UI: SSR + hydration | [leptos.dev](https://leptos.dev) |
| **leptos_axum** | Интеграция Leptos ↔ axum | — |
| **leptos_meta** | `<title>`, мета-теги, стили | — |
| **leptos_router** | Клиентский роутер | — |
| **tokio** | Async-рантайм | — |

## Движок сборки

- **cargo-leptos 0.3** — единая сборка: сервер (ssr), клиент (wasm/hydrate),
  Tailwind CSS → CSS (standalone-бинарник скачивается автоматически, версия —
  `LEPTOS_TAILWIND_VERSION`, по умолчанию v4.x).
- **cargo-chef** — кеширование зависимостей в Docker.
- **cargo-binstall** — быстрая установка инструментов (CI и Docker).
- Профиль `wasm-release` в `Cargo.toml` — минимизация WASM (opt-level `z`, LTO,
  codegen-units=1, panic=abort).

## Фронт

- Стили: **Tailwind CSS v4** (`style/main.css`, CSS-first конфигурация через
  `@theme`/`@source`) — собирается cargo-leptos, см. [asset-pipeline.md](asset-pipeline.md).
- `leptos_use` (0.19.x) — утилитарные хуки (`use_media_query` в сайд-меню).
- `gloo-net` — клиентские HTTP-запросы (запрос `/api/health` после гидратации).
- `thaw-ui` не используется: версия 0.4.x требует leptos 0.7 и несовместима с
  установленным leptos 0.8 (см. [roadmap.md](roadmap.md)).

## Тестирование и качество

- **Playwright** (`end2end/`) — e2e: главная, навигация, 404, `/api/health`;
  в CI — chromium, локально доступны firefox/webkit.
- **cargo fmt / clippy / test** — юнит-тесты конфигурации, коннекторов, `AppError`,
  версии.
- **WASM-гейт** — `cargo check --features hydrate --lib --target wasm32-unknown-unknown`.
- **cargo-audit** — аудит уязвимостей зависимостей (локально и в CI).
- **just** — типовые задачи: `just precommit` (fmt + clippy + tests + rustdoc).

## Среды исполнения

- Локальная разработка: Windows/Linux/macOS (см. [development.md](development.md)).
- Продакшн: Docker — сборка на Ubuntu, запуск на Alpine (см. [deployment.md](deployment.md)).