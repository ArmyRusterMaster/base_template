# Разработка / окружение

## Подготовка окружения

```bash
bash scripts/setup.sh      # Linux/macOS
```

Скрипт ставит: Docker + Compose, Node.js/npm, Rust (rustup) +
`wasm32-unknown-unknown`, `cargo-binstall`, `cargo-leptos`, `leptosfmt`, `just`,
`cargo-audit`, `git-cliff`, а также npm-зависимости e2e-тестов.

Два осознанных «не ставим»:

- **sccache** — падает на Windows при компиляции `web-sys`; в `.cargo/config.toml`
  проекта обёртка компилятора отключена (`rustc-wrapper = ""`);
- **tailwind CLI** — standalone-бинарник Tailwind скачивает сам cargo-leptos
  (см. [asset-pipeline.md](asset-pipeline.md)).

Те же инструменты нужны для `cargo fmt`, `cargo clippy`, `cargo audit`
(`rustup component add rustfmt clippy` делает setup-скрипт).

> Windows: используйте `just`/`cargo` из Git Bash либо PowerShell, но вместо
> `npm`/`npx` вызывайте `npm.cmd`/`npx.cmd` — политика выполнения может
> блокировать `*.ps1`-обёртки.

## Цикл разработки

```bash
just watch          # dev-сервер с live-reload (127.0.0.1:3000)
# или напрямую
cargo leptos watch
```

## Рецепты `just`

`justfile` — единая точка входа; `just --list` показывает все рецепты.

| Рецепт | Что делает |
|---|---|
| `just watch` | dev-сервер с live-reload |
| `just build` | релизная сборка: сервер (ssr) + WASM (hydrate) + CSS → `target/site` |
| `just serve` | запуск релизной сборки локально |
| `just fmt` / `just fmt-check` | форматирование / проверка формата (как в CI) |
| `just lint` | `cargo clippy --features ssr --all-targets -- -D warnings` |
| `just test` | юнит-тесты (`--features ssr`) + WASM-гейт |
| `just doc` | rustdoc без зависимостей |
| `just audit` | `cargo audit` |
| `just e2e` | Playwright (нужен запущенный сервер; можно `just e2e --project=firefox`) |
| `just docker` | `docker compose -f deploy/docker-compose.yaml up --build` |
| `just precommit` | fmt + clippy + tests + rustdoc |

## Сборка и запуск

```bash
just build                      # сервер + WASM + CSS → target/site + target/release
./target/release/base_template  # запуск сервера (порт из config/app.yaml)
```

## Качество

```bash
just precommit                  # быстрый полный прогон
# эквивалент вручную:
cargo fmt --all
cargo clippy --features ssr --all-targets -- -D warnings
cargo clippy -p core-shared --all-targets -- -D warnings      # крейт воркспейса
cargo test --features ssr
cargo test -p core-shared
cargo check --features hydrate --lib --target wasm32-unknown-unknown   # WASM-гейт
cargo doc --features ssr --no-deps
cargo audit
```

Команды с фичами выполняются из корня и относятся к пакету приложения; крейт
общих типов тестируется/линтится явно через `-p core-shared` (см.
[architecture.md](architecture.md) → «Воркспейс»).

CI гоняет то же самое плюс e2e, Docker и smoke — см. [deployment.md](deployment.md).

## E2E (Playwright)

```bash
just watch                     # сервер должен быть запущен
cd end2end
npm.cmd install                # Windows; на Linux/macOS — npm install
npx playwright install         # браузеры (chromium/firefox/webkit)
npx playwright test            # все движки
npx playwright test --project=chromium   # как в CI
```

- Адрес сервера берётся из `BASE_URL` (по умолчанию `http://127.0.0.1:3000`) —
  см. `end2end/playwright.config.ts`.
- Ожидаемые тексты (имя приложения, заголовок) — в `end2end/constants.ts` и
  переопределяются env `APP_NAME`/`APP_TITLE`.
- Что проверяется: SSR-разметка главной, клиентская навигация `/account` →
  `/status`, подгрузка коннекторов из `/api/health` после гидратации, страница
  404 и сам `/api/health` (+ заголовок `x-request-id`).
- В CI запускается только chromium; firefox/webkit — локально.