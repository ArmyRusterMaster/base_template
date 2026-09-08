# Архитектура проекта

## Структура

```
.
├── docs/                  # документация
├── config/app.yaml        # конфигурация приложения (yaml)
├── deploy/
│   ├── Dockerfile         # мультистейдж-сборка
│   └── docker-compose.yaml
├── end2end/               # Playwright e2e-тесты
├── public/                # статик-ассеты (копируются в site-root)
├── scripts/
│   └── setup.sh           # подготовка окружения
├── src/
│   ├── main.rs            # SSR-сервер (axum) + graceful shutdown
│   ├── lib.rs             # mod + hydrate()
│   ├── app.rs             # shell, App, страницы (Leptos-компоненты)
│   └── config.rs          # типизированная конфигурация
└── style/main.scss        # SCSS → CSS (через cargo-leptos)
```

## Потоки исполнения

### SSR (сервер, feature `ssr`)

1. `main.rs` загружает конфигурацию (`AppConfig::load`) — её адрес
   `server.host:server.port` становится точкой привязки.
2. `generate_route_list(App)` собирает маршруты из роутера Leptos.
3. `Router` монтирует `leptos_routes` — на каждый маршрут рендерится `shell`
   (HTML-каркас с hydration-скриптами).
4. `fallback(leptos_axum::file_and_error_handler)`:
   - отдаёт статику из site-root (JS/WASM/CSS),
   - 404-маршруты и ошибки возвращает в `shell`.

### Клиент (feature `hydrate`)

`lib.rs::hydrate()` монтирует тот же `App` поверх уже отрисованного сервером
DOM — переменные шаблоны на клиенте без полной перезагрузки.

### Конфигурация

`src/config.rs` — слои: `defaults` → `config/app.yaml` → `APP_*` env.
Валидация: типы serde (на этапе компиляции), значения — при старте (fail-fast).
Подробнее: [configuration.md](configuration.md).

## Компоненты Leptos

- `shell(options)` — HTML-обёртка (head, HydrationScripts, AutoReload, MetaTags).
- `App` — корневой компонент: stylesheet, title, роутер.
- `HomePage` — пример страницы с реактивным `RwSignal`.