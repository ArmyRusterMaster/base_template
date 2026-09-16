//! Общие типы (DTO) для SSR-сервера и WASM-клиента.
//!
//! Крейт сознательно **не зависит** от `leptos`, `axum` и `tokio`: он
//! компилируется и под `wasm32-unknown-unknown` (клиент), и под нативную цель
//! (сервер). Всё, что требует сети, файловой системы или async-рантайма, здесь
//! размещать нельзя — для этого есть серверные модули (`src/connectors`,
//! `src/error`, `src/logging`).
//!
//! Правило проекта (RULES.md §5): если тип используется и на фронтенде, и на
//! бэкенде, он живёт здесь, а не дублируется в двух местах.

use serde::{Deserialize, Serialize};

/// Снимок `GET /api/health`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HealthResponse {
    /// `"ok"` (все коннекторы живы) или `"degraded"`.
    pub status: String,
    /// Версия приложения в формате `vX.Y.Z-<hash>` (см. `src/version.rs`).
    pub version: String,
    /// Статусы коннекторов из реестра.
    #[serde(default)]
    pub connectors: Vec<ConnectorHealthDto>,
}

/// Статус одного коннектора в ответе health.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectorHealthDto {
    /// Идентификатор коннектора (`"echo"`, `"stripe"`, ...).
    pub id: String,
    /// Человекочитаемое описание.
    pub description: String,
    /// `"ok" | "degraded" | "down"` (см. `ConnectorStatus::as_str`).
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// JSON — контракт с клиентом: порядок полей и имена зафиксированы
    /// (на этом же формате завязан smoke-test в CI).
    const HEALTH_JSON: &str = r#"{"status":"ok","version":"v0.1.0-test","connectors":[{"id":"echo","description":"demo","status":"ok"}]}"#;

    #[test]
    fn health_response_serializes_to_expected_contract() {
        let value = HealthResponse {
            status: "ok".to_owned(),
            version: "v0.1.0-test".to_owned(),
            connectors: vec![ConnectorHealthDto {
                id: "echo".to_owned(),
                description: "demo".to_owned(),
                status: "ok".to_owned(),
            }],
        };

        let json = serde_json::to_string(&value).expect("сериализация DTO");
        assert_eq!(json, HEALTH_JSON);
    }

    #[test]
    fn health_response_round_trips() {
        let parsed: HealthResponse = serde_json::from_str(HEALTH_JSON).expect("десериализация DTO");

        assert_eq!(parsed.status, "ok");
        assert_eq!(parsed.version, "v0.1.0-test");
        assert_eq!(parsed.connectors.len(), 1);
        assert_eq!(parsed.connectors[0].id, "echo");
        assert_eq!(parsed.connectors[0].status, "ok");
    }

    #[test]
    fn connectors_field_is_optional_on_deserialize() {
        let parsed: HealthResponse =
            serde_json::from_str(r#"{"status":"degraded","version":"v0.0.0-dev"}"#)
                .expect("отсутствие поля connectors допустимо");

        assert!(parsed.connectors.is_empty());
    }
}
