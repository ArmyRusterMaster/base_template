# Разработка / окружение

## Подготовка окружения

`scripts/setup.sh` (Linux/macOS) автоматически ставит:

- Docker + Compose,
- Node.js/npm,
- Rust (rustup) + wasm32-unknown-unknown,
- cargo-leptos, leptosfmt, cargo-audit,
- npm-зависимости проекта.

Запуск:

```bash
bash scripts/setup.sh
```

> Windows: установить вручную Rust (rustup), `wasm32-unknown-unknown`,
> `cargo-leptos`, npm, Playwright. В PowerShell вместо `npx` используйте
> `npx.cmd` (политики выполнения могут блокировать `*.ps1`).

## Цикл разработки

```bash
# dev-сервер с live-reload (по умолчанию 127.0.0.1:3000)
cargo leptos watch

# или вручную
cargo-leptos watch
```

## Сборка и запуск

```bash
# релиз: сервер + WASM + CSS → target/site + target/release/base_template
cargo leptos build --release

# запуск сервера
./target/release/base_template
```

## Качество

```bash
cargo fmt --all            # форматирование
cargo clippy --features ssr --all-targets -- -D warnings
cargo test --features ssr  # юнит-тесты (конфигурация)
cargo audit                # аудит зависимостей
```

## E2E (Playwright)

```bash
cd end2end
npm install
npx playwright install       # установка браузеров
# сервер должен быть запущен на 127.0.0.1:3000
npx playwright test
```

Конфигурация: `end2end/playwright.config.ts` (chromium/firefox/webkit,
retries на CI). Тесты: `end2end/tests/`.