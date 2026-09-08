# Roadmap

План развития шаблона (исходный бэклог из `docs.md/docs.md` + новые пункты).

## Сделано

- [x] Graceful shutdown (Ctrl+C / SIGTERM).
- [x] Общий скрипт подготовки окружения (`scripts/setup.sh`).
- [x] Конфиги через YAML (config/app.yaml) + env (`APP_*`) с валидацией
      (compile-time типы serde + fail-fast при старте).
- [x] Dockerfile (мультистейдж, Ubuntu → Alpine, cargo-chef, cargo-leptos)
  и docker-compose.yaml.
- [x] CI/CD: `.github/workflows/ci.yaml` (fmt, clippy, тесты, релиз-сборка,
  e2e Playwright, Docker smoke).
- [x] Asset pipeline: cache-busting (cargo-leptos). Предсжатие `.gz`/`.br`
      убрано — сжатие выполняет внешний слой (Cloudflare/Nginx).
- [x] Секреты `.env` в `.gitignore`, `.env.example` для документирования.
- [x] Документация структурирована (`docs/`), добавлен `docs/current-state.md`.

## В работе / ближайшие

- [ ] Проверить сборку образа на реальном Docker (локально/CI).
- [ ] `cargo audit` в CI после установки (`cargo install cargo-audit`) —
  аудит зависимостей.
- [ ] Быстрый кеш: `cargo chef cook` и для WASM-зависимостей
  (`--features hydrate`), чтобы ускорить Docker-build.
- [ ] Playwright e2e: сделать `webServer` в `playwright.config.ts` и вынести
  порт/заголовок в конфиг.

## Среднесрочные

- [ ] Включить `leptos_use` и `leptos_fetch` в стек (см. [stack.md](stack.md)).
- [ ] UI-библиотека (thawui) для страниц администратора.
- [ ] Компиляция конфигурации на билд-тайме (`compile-time config`) —
  например, через `serde` + `include_str!`, если нужен статический
  конфиг для исключённых окружений.

## Бэклог из первоначального шаблона

- [ ] Деплой-пайплайн CD (публикация образа в реестр + автодеплой).