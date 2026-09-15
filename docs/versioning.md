# Версионирование и релизы

Источник правил — [RULES.md §6](../RULES.md). Здесь — как это устроено в коде и CI.

## Формат версии

`vX.Y.Z-<короткий хеш коммита>`, например `v0.1.0-f7eacc9`.

Хеш встраивается на этапе сборки (`build.rs` → `env!("GIT_HASH")` в
`src/version.rs`), приоритет источников:

1. переменная окружения `GIT_HASH` (Docker/CI задают её явно);
2. `git rev-parse --short=7 HEAD` (локальная сборка);
3. строка `dev` (если ни то, ни другое недоступно — например, в контейнере без
   `.git`).

Релиз = тег `vX.Y.Z` на `main`; во всех остальных сборках версия содержит хеш и
по ней всегда видно, из какого коммита получен артефакт.

## Где видна версия

| Место | Источник |
|---|---|
| Шапка сайта | `src/components.rs` → `version()` |
| Футер | `src/components.rs` → `version()` |
| `GET /api/health` | поле `version` (`src/main.rs`) |
| Лог старта сервера | `tracing::info!` в `src/main.rs` |
| Docker-образ | план: тег образа `vX.Y.Z-<hash>` (см. [roadmap.md](roadmap.md)) |

## Conventional commits — обязательны

Из сообщений коммитов строится CHANGELOG, поэтому формат обязателен:

- `feat:`, `fix:`, `chore:`, `docs:`, `refactor:`, `test:`, `ci:`, `build:`,
  `perf:`, `style:`;
- breaking-изменения — `feat!:` / `fix!:` (или `BREAKING CHANGE:` в теле);
- область — в скобках: `feat(ui): ...`.

Разбор сообщений настроен в [`cliff.toml`](../cliff.toml) (git-cliff v2).

## Релиз

```bash
# 1. main зелёный (CI: fmt, clippy, test, wasm-гейт, e2e, docker, smoke)
# 2. тег и пуш тега
git tag v0.2.0
git push origin v0.2.0
```

По тегу `v*` запускается джоба `release` (`.github/workflows/ci.yaml`):

1. `git-cliff` собирает `CHANGELOG.md` и коммитит его в `main`
   (`chore(release): ... [skip ci]`);
2. тем же инструментом формируется тело GitHub Release (`--latest --strip header`);
3. создаётся релиз с тегом и заметками.

> `CHANGELOG.md` — **генерируемый** файл. Правки вручную будут перезаписаны при
> следующем релизе.

## Локально

```bash
# весь CHANGELOG
git-cliff --config cliff.toml -o CHANGELOG.md

# заметки только для последнего тега
git-cliff --config cliff.toml --latest --strip header
```

`git-cliff` ставится скриптом окружения (`scripts/setup.sh`) или вручную:
`cargo binstall -y git-cliff` (либо `cargo install --locked git-cliff`).
