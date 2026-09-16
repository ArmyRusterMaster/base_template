# ADR-001: Публикация проверенного CI-образа в GHCR

- Status: accepted
- Date: 2026-09-16
- Owners: template owner
- Related code: `.github/workflows/ci.yaml`, `deploy/Dockerfile.prebuilt`
- Related plan: [build-pipeline-and-tech-debt](../plans/2026-09-16-build-pipeline-and-tech-debt.md)

## Context

Владелец выбрал GHCR. CI уже сохраняет Docker-образ для smoke-тестов;
повторная сборка при публикации могла бы дать другой образ.

## Decision

После smoke, e2e и audit публиковать сохранённый образ без пересборки.
Только push стабильного тега vX.Y.Z; теги образа: версия, версия с 12 символами
SHA, latest. GitHub Release зависит от публикации. Реестр — имя репозитория
в нижнем регистре. Автодеплоя нет.

## Alternatives considered

- Docker Hub: не выбран владельцем.
- Пересборка в publish: не гарантирует совпадения с проверенным образом.
- PAT: избыточен при доступном GITHUB_TOKEN.

## Consequences

Нужен доступ Actions к GHCR package; пререлизы пока отклоняются.
Повторный релиз может переместить latest; теги изменяемы, для деплоя нужен digest.
Публикация нескольких тегов не атомарна. Артефакт хранится один день.

## Security impact

Токен передаётся через stdin, packages: write выдан только publish.
PR не публикуют образ. Остальные write-права ограничены fmt/release.

## Performance impact

Публикация не запускает компиляцию; измерения производительности не проводились.

## Migration and rollback

Существующие локальные теги не меняются. Для отката развернуть предыдущий
проверенный digest; отключение publish не влияет на сборку и тесты.

## Verification

Локальный review зависимостей джоб и shell-кода. Удалённая публикация и доступ
к GHCR требуют проверки первым релизом; успешный CI пока не заявляется.
