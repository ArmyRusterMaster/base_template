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
  SСSS → CSS (через downloaded dart-sass).
- **cargo-chef** — кеширование зависимостей в Docker.
- Профиль `wasm-release` в `Cargo.toml` — минимизация WASM (opt-level `z`, LTO,
  codegen-units=1, panic=abort).

## Фронт

- Стили: SCSS (`style/main.scss`), собирается в CSS движком cargo-leptos (dart-sass).
- (Планы) UI-библиотеки: [thawui](https://thawui.vercel.app/), `leptos_use`,
  `leptos_fetch` — см. [roadmap.md](roadmap.md).

## Тестирование и качество

- **Playwright** (`end2end/`) — e2e (chromium/firefox/webkit).
- **cargo fmt / clippy / test** — юнит-тесты конфигурации.
- **cargo-audit** — аудит уязвимостей зависимостей.

## Среды исполнения

- Локальная разработка: Windows/Linux/macOS (см. [development.md](development.md)).
- Продакшн: Docker — сборка на Ubuntu, запуск на Alpine (см. [deployment.md](deployment.md)).