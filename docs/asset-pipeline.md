# Asset pipeline (сборка и отдача статики)

Сборка и отдача статики — из коробки через cargo-leptos, без ручной настройки.

## Tailwind CSS v4

Входной файл — `style/main.css`, указан в `Cargo.toml`:

```toml
tailwind-input-file = "style/main.css"
```

Что делает cargo-leptos:

1. скачивает standalone-бинарник Tailwind (версия — `LEPTOS_TAILWIND_VERSION`,
   по умолчанию v4.x) в свои временные каталоги;
2. запускает его по входному файлу: `@import "tailwindcss"` подтягивает
   фреймворк, `@source "../src"` указывает, где искать классы (они живут в
   `view!`-макросах Rust);
3. прогоняет результат через Lightning CSS и кладёт в
   `target/site/pkg/base_template.css` — этот путь прописан в `<Stylesheet>` в
   `src/app.rs`.

JS-конфиг (`tailwind.config.js`) в v4 не нужен: тема описывается директивой
`@theme`, источники — `@source`. Глобальный `tailwindcss` CLI ставить не нужно
(и не рекомендуется: версия может разойтись с той, что ожидает cargo-leptos) —
`scripts/setup.sh` его не ставит.

## Cache-busting (хэши в именах)

В `release`-сборке cargo-leptos автоматически генерирует файлы с хэшем
(например, `base_template-<hash>.js/.wasm/.css`) и подставляет актуальные имена
в HTML. Браузеры кешируют их «навсегда», а после релиза ссылки обновляются —
старый UI не залипает. Пути прописаны в `[package.metadata.leptos]`
(tailwind-input-file, assets-dir, site-root) — править их нужно только там.

## Сжатие (gzip/brotli)

Приложение **не сжимает статику само**: предобработка `.gz`/`.br` убрана из
сборки (раньше это делал `scripts/precompress.sh`). Если сжатие нужно — разумнее
отдать его «впереди стоящему» слою:

- **Cloudflare** — brotli по умолчанию на крае;
- **Nginx** — `gzip on;` (+ `ngx_brotli`);
- **Caddy** — `encode zstd gzip`.

Технически `leptos_axum::file_and_error_handler` умеет отдавать предсжатые
копии, если они есть рядом с файлом (`ServeDir::precompressed_gzip/br`), — так
что при необходимости можно вернуть предобработку без изменений в коде.