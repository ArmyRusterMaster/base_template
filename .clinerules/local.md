# Global Rust Engineering Rules

## Приоритеты

Соблюдай приоритеты в таком порядке:

1. Корректность и безопасность.
2. Явное сохранение инвариантов и безопасный control flow.
3. Максимальная эффективность кода без потери читаемости.
4. Тестируемость и наблюдаемость.
5. Обратная совместимость публичного API.
6. Минимальный и понятный diff.
7. Документация и сопровождение.

Оптимизация не должна превращать код в нечитабельный, хрупкий или трудно тестируемый код. Не выполняй микрооптимизацию без измерения, профиля или явно подтверждённого hot path.

## Исследование проекта

Перед изменением:

1. Изучи `Cargo.toml`, workspace, edition и toolchain.
2. Прочитай `README.md`, `CONTRIBUTING.md`, `docs/` и локальные Cline rules.
3. Найди похожие типы, ошибки, middleware, handlers, тесты и benchmark-код.
4. Проверь `git status` и `git diff`.
5. Определи публичные API, границы доверия и возможные изменения протокола.
6. Не выдумывай версии crate, feature flags, endpoint-ы, схемы или требования.

## Идиоматичные паттерны Rust

### Newtype

Используй newtype для значений, которые имеют одинаковое физическое представление, но разный смысл:

```rust
struct UserId(uuid::Uuid);
struct OrderId(uuid::Uuid);
struct Email(String);
struct NonEmptyString(String);
struct Port(u16);
```

Не смешивай `UserId`, `OrderId`, деньги, единицы измерения, токены, внешние и внутренние идентификаторы через `String`, `u64` или `Uuid`.

Для newtype явно решай:

- нужны ли `From`/`TryFrom`;
- должен ли внутренний тип быть скрыт;
- нужна ли валидация;
- как выполняется `Serialize`/`Deserialize`;
- можно ли безопасно логировать значение;
- должна ли операция быть zero-cost.

Не добавляй `Deref` автоматически: используй его только если newtype действительно должен вести себя как обёрнутый тип.

### Типы состояний

Используй typestate, когда неправильная последовательность операций должна быть невозможна на уровне компиляции:

```rust
struct Connection<State> {
    inner: InnerConnection,
    state: State,
}

struct Disconnected;
struct Connected;

impl Connection<Disconnected> {
    fn connect(self) -> Result<Connection<Connected>, Error> {
        // ...
    }
}

impl Connection<Connected> {
    fn send(&mut self, request: Request) -> Result<Response, Error> {
        // ...
    }
}
```

Применяй typestate для:

- незаполненной и валидной конфигурации;
- authenticated/unauthenticated состояния;
- открытого и закрытого ресурса;
- подготовленного и неподготовленного запроса;
- staged workflow;
- обязательного вызова методов в определённом порядке.

Не используй typestate, если количество состояний чрезмерно усложняет API или типы начинают доминировать над бизнес-логикой. В таком случае используй runtime-проверку с ясной ошибкой.

### Builder

Используй builder для сложных конструкций с несколькими опциональными полями, коллекциями, валидацией или side effects.

Правила:

- обязательные значения принимай в `new`;
- опциональные значения настраивай методами;
- методы должны иметь предметные имена;
- terminal method должна называться `build`, `finish`, `spawn`, `send` или другим точным глаголом;
- валидацию выполняй в terminal method;
- возвращай typed error, а не строку;
- consuming builder используй, когда передаётся владение;
- mutable builder используй, когда builder должен удобно изменяться условно;
- не создавай builder для двух простых полей.

### Enum и state machine

Используй `enum`, когда множество состояний или вариантов известно заранее:

```rust
enum PaymentState {
    Pending,
    Authorized,
    Captured,
    Failed { reason: FailureReason },
}
```

Проверяй exhaustiveness через `match`. Не используй `String` для закрытого набора вариантов.

Для открытого набора, который приходит извне, используй отдельный `Unknown(String)` или аналогичный fallback, если это нужно для совместимости.

### Value objects

Выделяй отдельные типы для:

- денежных сумм и валют;
- URL и нормализованных URL;
- email;
- времени с явно указанной семантикой;
- лимитов, размеров и квот;
- версий;
- ролей и разрешений;
- нормализованных путей;
- validated identifiers.

Валидация должна происходить как можно ближе к границе системы.

### Ошибки

- Используй `Result` вместо panic для ожидаемых ошибок.
- Используй `thiserror` или эквивалент для библиотечных error types.
- Используй `anyhow` только на application boundary, если это соответствует проекту.
- Добавляй контекст к IO, сети, БД, сериализации и конфигурации.
- Не сравнивай ошибки по тексту.
- Не теряй исходную ошибку.
- Не логируй одну и ту же ошибку на каждом слое.
- Ошибки не должны раскрывать секреты, токены, внутренние пути или персональные данные.

### Iterator, ownership и allocation

- Используй итераторы, если они повышают ясность и не создают лишних аллокаций.
- Избегай промежуточных `collect`, если можно передать iterator дальше.
- Используй `&str` вместо `String`, когда владение не требуется.
- Используй `Cow` только когда он делает ownership trade-off яснее.
- Не вызывай `clone()` как средство обхода ownership без анализа стоимости.
- Резервируй capacity для коллекций, если размер известен или хорошо оценивается.
- Используй `Vec::with_capacity`, `String::with_capacity` и `HashMap::with_capacity` только при наличии обоснования.
- Не оптимизируй обычный код за счёт сложного unsafe.
- Для горячих циклов учитывай количество аллокаций, копирований, виртуальных вызовов и блокировок.
- Не используй `Box<dyn Trait>` в hot path без понимания стоимости динамической диспетчеризации.
- Обобщения и статическую диспетчеризацию предпочитай там, где это улучшает измеренную производительность.
- Не избегай динамической диспетчеризации догматично: читаемость и размер бинарника тоже важны.

### Async и concurrency

- Отличай IO-bound и CPU-bound работу.
- Не выполняй блокирующую работу внутри async runtime.
- Не используй `std::thread::sleep` в async-коде.
- Не удерживай mutex guard через `.await`.
- Не удерживай `tracing::Span::enter()` guard через `.await`; используй `#[instrument]` или `.instrument(span)`.
- Добавляй timeout, cancellation и backpressure там, где они необходимы.
- Каждая background task должна иметь владельца, lifecycle и стратегию завершения.
- Не создавай неограниченные каналы без доказанной причины.
- Учитывай fairness, starvation, deadlock и порядок захвата lock-ов.
- Не используй `Arc<Mutex<T>>` автоматически: сначала проверь ownership, actor model, message passing или immutable shared state.
- Не делай `spawn_blocking` заменой архитектурного решения для длительной CPU-работы.
- Не допускай бесконтрольного fan-out задач.

## Эффективность без потери читаемости

Перед оптимизацией:

1. Определи пользовательский или системный SLA.
2. Найди измеримый bottleneck.
3. Зафиксируй baseline.
4. Измерь изменение.
5. Проверь регрессию памяти и latency.
6. Оставь комментарий, если оптимизация нетривиальна.

Проверяй:

- аллокации;
- копирования;
- размер структур;
- cache locality;
- lock contention;
- количество syscalls;
- сериализацию;
- сетевые round trips;
- batch operations;
- размер payload;
- повторные вычисления;
- работу с `String`, `Vec`, `HashMap`;
- async scheduling и blocking sections.

Не принимай benchmark за доказательство улучшения, если benchmark не моделирует реальную нагрузку.

## Документация

### Публичный Rust API

Для каждого публичного типа, trait, функции, метода и модуля:

- добавляй rustdoc;
- объясняй назначение и инварианты;
- документируй ошибки;
- документируй panic conditions;
- документируй safety для unsafe;
- добавляй пример, если API нетривиален;
- отмечай ownership, lifetime и concurrency assumptions;
- не обещай поведение, которое код не гарантирует.

Проверяй:

```bash
cargo doc --workspace --no-deps
cargo test --doc --workspace
```

Не используй `#[allow(missing_docs)]` глобально без явной причины.

### Архитектурная документация

Поддерживай актуальность:

- `README.md`;
- `docs/architecture.md`;
- `docs/decisions/`;
- `CHANGELOG.md`;
- migration notes;
- API contract;
- runbook;
- threat model;
- performance notes.

После изменения архитектуры, публичного поведения, конфигурации, API или протокола обновляй соответствующую документацию в том же diff.

### API documentation

Для Web API документируй:

- endpoint;
- метод и URL;
- authentication и authorization;
- request schema;
- response schema;
- status codes;
- error schema;
- pagination;
- filtering;
- sorting;
- idempotency;
- rate limits;
- timeout behavior;
- retry semantics;
- consistency guarantees;
- versioning;
- deprecation policy;
- security assumptions.

## Structured logging и tracing

Используй `tracing` вместо разбросанных `println!`.

- `trace`: очень подробная диагностика;
- `debug`: разработческие детали;
- `info`: значимые бизнес- и lifecycle-события;
- `warn`: восстановимые проблемы;
- `error`: операция завершилась ошибкой.

Используй:

```rust
#[tracing::instrument(
    skip(state, payload),
    fields(request_id = %request_id, user_id = %user_id)
)]
async fn handler(...) -> Result<Response, ApiError> {
    tracing::info!(operation = "create_order", "request started");
    // ...
}
```

Правила:

- логируй структурированные поля;
- используй request ID, correlation ID и operation name;
- не логируй пароли, токены, cookies, API keys, полные authorization headers;
- маскируй email, phone, IP и персональные данные по policy;
- не логируй весь request/response body по умолчанию;
- не добавляй высококардинальные поля без необходимости;
- не создавай шумные `info`-события внутри tight loop;
- используй `debug` или sampling для частых событий;
- логируй начало и завершение важных операций;
- указывай duration, result и error category;
- добавляй причины отказа без раскрытия внутренней информации.

Для library crate эмитируй tracing events, но не устанавливай global subscriber. Subscriber настраивает executable/application.

## Web API security

### Control flow

Для каждого handler-а и критической функции отдельно проверяй:

1. authentication;
2. authorization;
3. input validation;
4. resource ownership;
5. rate limit;
6. idempotency;
7. business invariants;
8. side effects;
9. transaction boundary;
10. audit event;
11. response filtering.

Authorization должна проверяться на сервере и быть привязана к ресурсу, а не только к наличию роли.

Не полагайся на:

- скрытые поля UI;
- URL parameters без проверки владельца;
- client-provided role;
- client-provided status;
- client-provided price;
- client-provided tenant ID;
- порядок вызова endpoint-ов;
- frontend validation.

### Input и output

- Ограничивай размер body, query, headers и uploads.
- Валидируй UTF-8, диапазоны, длины, enum values и nested structures.
- Отделяй внешние DTO от внутренних domain models.
- Не возвращай внутреннюю модель БД напрямую.
- Делай explicit response DTO.
- Не раскрывай stack trace, SQL, файловые пути и внутренние идентификаторы.
- Используй allowlist для сортировки, фильтрации, redirect URL и полей.
- Защищайся от mass assignment.
- Не формируй SQL, shell commands, paths или HTML из непроверенного ввода.
- Для внешних URL проверяй SSRF, redirect chains и private network targets.
- Для файлов проверяй размер, тип, имя и destination path.
- Для pagination ограничивай page size.

### HTTP semantics

- Используй правильные status codes.
- Сделай error response стабильным и документированным.
- Не меняй смысл status code ради удобства клиента.
- Для повторяемых операций поддерживай idempotency keys, если это нужно.
- Для retries учитывай duplicate side effects.
- Добавляй timeout на внешние вызовы.
- Для mutation используй transaction или явную компенсацию.
- Проверяй race conditions между authorization и side effect.
- Учитывай replay, duplicate request, stale data и concurrent updates.

### Secrets и supply chain

- Не добавляй секреты в код, логи, snapshots, fixtures и workflow output.
- Используй GitHub secrets или secret manager.
- Закрепляй GitHub Actions по commit SHA для production-critical workflows, если policy проекта это требует.
- Ограничивай `GITHUB_TOKEN` через минимальные permissions.
- Не запускай непроверенный код из pull request в окружении с production secrets.
- Проверяй зависимости через `cargo audit`, `cargo deny` и lockfile review.
- Не используй dependency без проверки maintenance, лицензии и security history.

## Testing

Добавляй:

- unit tests для domain logic;
- integration tests для boundaries;
- contract tests для Web API;
- property-based tests для сложных инвариантов;
- regression tests для найденных ошибок;
- concurrency tests для shared state;
- timeout/cancellation tests для async;
- authorization tests для каждого защищённого ресурса;
- negative tests для invalid input;
- fuzzing для parser-ов, декодеров и boundary input, если это оправдано.

Тестируй не только happy path, но и:

- пустые значения;
- максимальные размеры;
- duplicate requests;
- race conditions;
- partial failure;
- timeout;
- cancellation;
- malformed payload;
- unexpected enum variants;
- unauthorized и forbidden cases;
- tenant isolation.

## Debugging и диагностика

При ошибке:

1. Зафиксируй точную команду и окружение.
2. Воспроизведи проблему.
3. Сохрани полный compiler/test/runtime output.
4. Уменьши проблему до минимального сценария.
5. Проверь control flow и ownership.
6. Проверь конфигурацию и feature flags.
7. Добавь временную диагностическую instrumentation.
8. Исправь первопричину.
9. Добавь regression test.
10. Удали временный шумный debug-код.
11. Повтори узкие и полные проверки.

Не исправляй проблему через бездумный `unwrap`, `clone`, `allow`, отключение теста или изменение timeout без понимания причины.

## Профилирование и benchmarks

Переходи к profiling и benchmarks только после того, как приложение функционально стабильно:

1. Пройди unit, integration и end-to-end tests.
2. Устрани flaky tests и очевидные correctness bugs.
3. Зафиксируй baseline.
4. Определи workload.
5. Добавь Criterion benchmark для стабильной функции или алгоритма.
6. Измерь throughput, latency, allocations и memory footprint.
7. Используй profiler для поиска hot paths.
8. Измени код минимально.
9. Повтори benchmark и profiling.
10. Проверь читаемость и отсутствие регрессий.
11. Документируй измеренное улучшение.

Не называй код «оптимизированным» без сравнения с baseline.

Примеры команд:

```bash
cargo bench
cargo test --release
cargo flamegraph --bin <binary>
cargo instruments
perf stat -- target/release/<binary>
perf record --call-graph dwarf target/release/<binary>
```

Конкретный инструмент выбирай по ОС и проекту.

## Обязательные проверки

Минимум:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo doc --workspace --no-deps
```

Для изменённых зависимостей:

```bash
cargo audit
cargo deny check
cargo tree -d
```

Для unsafe, parser-ов и boundary code добавляй соответствующие дополнительные проверки.

## Git

- Не перезаписывай чужие изменения.
- Не делай `reset --hard`, `clean`, force push или удаление веток без разрешения.
- Не создавай commit без явной просьбы.
- Не меняй несвязанные файлы.
- Не скрывай незакоммиченные изменения.
- Проверяй diff перед и после работы.
- В commit/PR описывай risk, testing и migration impact.

## Living Project Documentation for AI Agents

### Обязательный принцип

Документация проекта является частью инженерного результата. Если изменение меняет поведение, архитектуру, API, поток данных, инварианты, безопасность, performance characteristics или планы проекта, обновляй документацию в том же diff.

Не завершай задачу с кодом, который противоречит документации.

### Точка входа

При входе в проект сначала найди и прочитай:

1. `AGENTS.md`, если существует.
2. `docs/README.md`, если существует.
3. `docs/project-state.md`.
4. `docs/architecture.md` или релевантный документ архитектуры.
5. Документацию изменяемой подсистемы.
6. `docs/agents/current-task.md`, если существует.
7. Незакоммиченные изменения и активные планы.

Если документация отсутствует, не создавай большую систему вслепую: сначала составь минимальную карту проекта и добавь её в предусмотренное место.

### Источники истины

Разделяй:

- фактически реализованное поведение;
- запланированное поведение;
- временные решения;
- гипотезы;
- нерешённые вопросы;
- сознательно отложенные решения.

Никогда не описывай план как уже реализованную функциональность.

Для каждого утверждения о текущем состоянии по возможности указывай:

- источник кода;
- модуль или файл;
- дату проверки;
- статус: `current`, `planned`, `deprecated`, `unknown`.

Код является источником истины для фактического поведения, а документация — индексом, объяснением архитектуры, ограничений и решений.

### Обязательное чтение перед изменением

Перед изменением подсистемы ответь для себя:

- за что она отвечает;
- какие у неё входы и выходы;
- какие есть владельцы данных;
- какие инварианты должны сохраняться;
- какие внешние системы затрагиваются;
- где находятся границы доверия;
- какие ошибки возможны;
- какие тесты и команды проверяют поведение;
- какие документы могут устареть после изменения.

### Обновление документации после изменения

Проверь и обнови, если применимо:

- `docs/project-state.md`;
- `docs/architecture.md`;
- документацию изменённой подсистемы;
- документацию flow или sequence;
- ADR;
- API documentation;
- `docs/security.md`;
- `docs/performance.md`;
- `docs/plans/`;
- `CHANGELOG.md`;
- `docs/agents/current-task.md`;
- `docs/agents/change-log.md`;
- `docs/agents/working-memory.md`.

Не обновляй документы механически. Обновляй только те части, которые действительно изменились.

### Текущее состояние проекта

`docs/project-state.md` должен отвечать:

- что проект делает сейчас;
- какие компоненты существуют;
- что работает стабильно;
- что находится в разработке;
- какие известные ограничения существуют;
- какие зависимости критичны;
- какие интеграции активны;
- какие проверки проходят;
- какие риски остаются.

В документе явно разделяй:

```markdown
## Implemented

## In progress

## Planned

## Known limitations

## Open risks

## Last verified
```

### Планирование

Для нетривиальной задачи создай или обнови документ в `docs/plans/`.

План должен содержать:

```markdown
# Название плана

## Goal

## Non-goals

## Current state

## Proposed design

## Affected components

## Invariants

## Security impact

## Performance impact

## Migration

## Steps

## Verification

## Risks

## Open questions

## Status
```

Статусы используй только из набора:

- `proposed`;
- `accepted`;
- `in_progress`;
- `blocked`;
- `completed`;
- `superseded`;
- `cancelled`.

После каждого существенного этапа обновляй статус и список выполненных шагов.

### ADR

Создавай ADR в `docs/decisions/`, если решение:

- меняет границы компонентов;
- добавляет существенную зависимость;
- изменяет публичный API или протокол;
- влияет на безопасность;
- влияет на persistence или миграцию;
- создаёт долгосрочный performance trade-off;
- меняет async/concurrency model;
- выбирает один из нескольких архитектурных подходов.

Формат ADR:

```markdown
# ADR-NNN: Название решения

- Status: proposed | accepted | deprecated | superseded
- Date: YYYY-MM-DD
- Owners: ...
- Related code: ...
- Related plan: ...

## Context

## Decision

## Alternatives considered

## Consequences

## Security impact

## Performance impact

## Migration and rollback

## Verification
```

Не создавай ADR для очевидного локального исправления.

### Агентская рабочая память

Файл `docs/agents/working-memory.md` должен содержать только долговременный контекст:

- архитектурные инварианты;
- зафиксированные соглашения;
- важные ограничения;
- команды проверки;
- known pitfalls;
- решения, которые нельзя отменять без review;
- ссылки на canonical documentation.

Не записывай туда временные рассуждения, полные логи, секреты или устаревшие гипотезы.

Файл `docs/agents/current-task.md` должен содержать только активную задачу:

```markdown
# Current Task

## Goal

## Scope

## Out of scope

## Current status

## Completed

## In progress

## Next steps

## Blockers

## Files touched

## Checks run

## Decisions made

## Risks

## Last updated
```

После завершения задачи:

1. Перенеси устойчивые решения в ADR или документацию.
2. Обнови `project-state.md`.
3. Очисти `current-task.md` или пометь задачу завершённой.
4. Добавь краткую запись в `change-log.md`.
5. Не оставляй там временный шум.

### Change log для агентов

`docs/agents/change-log.md` должен быть кратким:

```markdown
## YYYY-MM-DD — Краткое название

- Changed:
- Why:
- Files:
- Tests:
- Docs:
- Risks:
```

Записывай только изменения, полезные будущему агенту. Не копируй туда весь git diff.

### Проверка согласованности

Перед завершением задачи проверь:

- документация описывает текущий код;
- ссылки на файлы и команды существуют;
- статусы планов актуальны;
- ADR не противоречит реализации;
- API docs совпадают со схемами и status codes;
- security docs отражают реальные trust boundaries;
- performance docs не содержат неподтверждённых заявлений;
- временные заметки не выданы за факты;
- `current-task.md` отражает фактический прогресс.

Если не удалось проверить утверждение, пометь его как `unknown` или `needs verification`.

### Отчёт агента

Финальный ответ всегда должен содержать:

1. Что сделано.
2. Что не сделано.
3. Какие документы обновлены.
4. Какие планы и статусы изменились.
5. Какие решения приняты.
6. Какие проверки запущены.
7. Какие риски и открытые вопросы остались.
8. Следующий рекомендуемый шаг.

Не заявляй о проверке, которую фактически не выполнял.