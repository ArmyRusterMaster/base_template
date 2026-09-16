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

## Рецепты `just` — опционально, прогон в CI

`justfile` остаётся в репозитории как **декларация задач**, но локально `just`
не ставится (осознанное решение — не загромождать машину посторонним ПО), а его
роль на себя берёт CI: джобы `fmt`/`clippy`/`test`/`build-release`/`e2e`/`docker`
прогоняют эквиваленты рецептов на каждый push.

Если `just` всё же установлен локально (`cargo binstall -y just`), рецепты
работают как описано ниже; в противном случае пользуйтесь прямыми
cargo-командами из раздела «Качество».

| Рецепт | Что делает | CI-эквивалент |
|---|---|---|
| `just watch` | dev-сервер с live-reload | — |
| `just build` | релизная сборка: сервер (ssr) + WASM (hydrate) + CSS → `target/site` | `build-release` |
| `just serve` | запуск релизной сборки локально | — |
| `just fmt` / `just fmt-check` | форматирование / проверка формата (как в CI) | `fmt` |
| `just lint` | `cargo clippy --features ssr --all-targets -- -D warnings` + core-shared | `clippy` |
| `just test` | юнит-тесты (`--features ssr`) + core-shared + WASM-гейт | `test` |
| `just doc` | rustdoc без зависимостей | — |
| `just audit` | `cargo audit` | `audit` |
| `just e2e` | Playwright (нужен запущенный сервер; можно `just e2e --project=firefox`) | `e2e` |
| `just docker` | `docker compose -f deploy/docker-compose.yaml up --build` | `docker`/`smoke` |
| `just precommit` | fmt + clippy + tests + rustdoc | весь конвейер |

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