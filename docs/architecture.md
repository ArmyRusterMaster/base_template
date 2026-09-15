# Архитектура проекта

## Структура

```
.
├── AGENTS.md              # точка входа для AI-агентов (карта проекта)
├── cliff.toml             # конфигурация git-cliff (CHANGELOG/релизы) — см. versioning.md
├── justfile               # типовые задачи разработки
├── config/app.yaml        # конфигурация приложения (yaml)
├── deploy/
│   ├── Dockerfile         # мультистейдж-сборка (Ubuntu → Alpine)
│   ├── Dockerfile.prebuilt# рантайм-образ из готового артефакта (CI)
│   └── docker-compose.yaml
├── docs/                  # документация (+ docs/agents/ — заметки агентов)
├── end2end/               # Playwright e2e-тесты (specs + constants.ts)
├── public/                # статик-ассеты (копируются в site-root)
├── scripts/setup.sh       # подготовка окружения
├── src/
│   ├── main.rs            # SSR-сервер (axum), /api/health, request-id, shutdown
│   ├── lib.rs             # mod + hydrate()
│   ├── app.rs             # shell, App, маршруты и страницы (Leptos-компоненты)
│   ├── components.rs      # UI-скелет: Layout, Header, Sidebar, Footer
│   ├── config.rs          # типизированная конфигурация (yaml + APP_*)
│   ├── dto.rs             # DTO, общие для клиента и сервера (план: crates/core-shared)
│   ├── error.rs           # AppError + IntoResponse (feature `ssr`)
│   ├── logging.rs         # инициализация tracing (feature `ssr`)
│   ├── version.rs         # версия vX.Y.Z-<hash> (GIT_HASH из build.rs)
│   └── connectors/mod.rs  # трейт Connector + ConnectorRegistry (feature `ssr`)
└── style/main.css         # Tailwind CSS v4 → CSS (через cargo-leptos)
```

## Слои и модули

| Слой | Модули | Ответственность |
|---|---|---|
| Конфигурация | `config.rs` | слои `defaults` → `config/app.yaml` → `APP_*`, fail-fast-валидация |
| Наблюдаемость | `logging.rs`, `version.rs` | `tracing`-подписчик, версия сборки `vX.Y.Z-<hash>` |
| Ошибки | `error.rs` | единый `AppError` для handler-ов axum; наружу — только безопасный JSON |
| Интеграции | `connectors/mod.rs` | контракт `Connector` + реестр; конкретные интеграции — в наследниках |
| Общие типы | `dto.rs` | DTO запросов/ответов, используемые и SSR, и hydrate |
| UI | `app.rs`, `components.rs` | shell, маршруты, страницы, layout |
| Стили | `style/main.css` | Tailwind v4 (CSS-first), собирается cargo-leptos |

Модули, помеченные в `lib.rs` как `#[cfg(feature = "ssr")]`, не компилируются в
клиентский бандл (WASM).


## Потоки исполнения

### SSR (сервер, feature `ssr`)

1. `main.rs` инициализирует логирование (`logging::init`), затем загружает
   конфигурацию (`AppConfig::load`) — её адрес `server.host:server.port` становится
   точкой привязки.
2. `generate_route_list(App)` собирает маршруты из роутера Leptos.
3. `Router` монтирует:
   - `GET /api/health` — инфраструктурный health (версия + статусы коннекторов);
   - `leptos_routes` — на каждый маршрут рендерится `shell` (HTML-каркас с
     hydration-скриптами);
   - `fallback(leptos_axum::file_and_error_handler)` — статика из site-root
     (JS/WASM/CSS) и 404 → `shell`.
4. Слой `request-id` (`tower-http`) ставит `x-request-id` на каждый запрос и
   возвращает его в ответе.
5. Завершение — `shutdown_signal()` (Ctrl+C / SIGTERM) → graceful shutdown.

### Клиент (feature `hydrate`)

`lib.rs::hydrate()` монтирует тот же `App` поверх уже отрисованного сервером
DOM — переходы между страницами без полной перезагрузки. Страница `/status` и
блок «Подключения» догружают данные из `/api/health` уже на клиенте
(`Effect` + `spawn_local` из `leptos::task`).

### Стили

`style/main.css` (Tailwind v4, CSS-first) собирается cargo-leptos в
`target/site/pkg/base_template.css`; этот же путь указан в `<Stylesheet>` в
`src/app.rs`. Подробности — [asset-pipeline.md](asset-pipeline.md).

### Конфигурация

`src/config.rs` — слои: `defaults` → `config/app.yaml` → `APP_*` env.
Валидация: типы serde (на этапе компиляции), значения — при старте (fail-fast).
Подробнее: [configuration.md](configuration.md).

## Компоненты Leptos

- `shell(options)` — HTML-обёртка (head, HydrationScripts, AutoReload, MetaTags).
- `App` — корневой компонент: stylesheet, title, роутер с маршрутами `/`,
  `/account`, `/status` и fallback-страницей 404.
- `Layout` / `Header` / `Sidebar` / `Footer` (`src/components.rs`) — общий каркас
  страниц; сайд-меню адаптивное (`leptos_use::use_media_query`).
- `HomePage`, `AccountPage`, `StatusPage`, `NotFound`, `ConnectorsList` — страницы
  и блок статусов коннекторов.

## Правила при работе с leptos 0.8 / leptos_router 0.8

- `A` живёт в `leptos_router::components` и **не принимает `class` как prop** —
  классы передаются attribute spreading: `<A href="/" attr:class="...">`.
- Пути маршрутов задаются макросом `path!`: `path!("")`, `path!("/account")`.
- Несендовые future (сигналы) запускаются через `leptos::task::spawn_local`,
  потокобезопасные — через `leptos::task::spawn` (в prelude их нет — импортируйте явно).
