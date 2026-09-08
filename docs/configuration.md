# Конфигурация проекта

Конфигурация строится из **трёх слоёв** (от слабого к сильному):

1. дефолты в `src/config.rs` (`AppConfig::default()`);
2. файл `config/app.yaml`;
3. переменные окружения `APP_*`.

Значения из старших слоёв перекрывают младшие, отсутствующие поля подтягиваются
из дефолтов.

## Валидация

- **На этапе компиляции** — типы проверяются `serde::Deserialize`; структура
  конфигурации строгая: любой неизвестный ключ → ошибка (`deny_unknown_fields`).
- **На старте** — значения валидируются `AppConfig::validate()` (fail-fast):
  сервер **не поднимется** с невалидным IP или портом, а выведет понятную
  ошибку и завершится с кодом 1.

## Файл `config/app.yaml`

```yaml
app_name: "base_template"

server:
  host: "127.0.0.1"   # 0.0.0.0 — для Docker/CI
  port: 3000
```

## Переменные окружения

Префикс `APP_`. Переопределяют файл конфигурации.

| Переменная | Тип | Описание |
|---|---|---|
| `APP_APP_NAME` | `string` | Имя приложения (логи) |
| `APP_SERVER_HOST` | `IP` | IP для слушания; валидируется как `IpAddr` |
| `APP_SERVER_PORT` | `u16` 1..65535 | Порт HTTP |

Пример:

```bash
export APP_SERVER_HOST=0.0.0.0
export APP_SERVER_PORT=8080
```

Полный список переменных: `.env.example`.

## Параметры Leptos (Cargo.toml / `LEPTOS_*`)

`[package.metadata.leptos]` в `Cargo.toml` задаёт пути сборки (site-root,
style-file, assets-dir, фичи bin/lib, профиль wasm-release). При необходимости их
можно переопределить переменными `LEPTOS_*` (например, `LEPTOS_SITE_ROOT` на
продакшене). В Docker это делает ENV в `deploy/Dockerfile`.