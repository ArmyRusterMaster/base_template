//! DTO, общие для клиента и сервера.
//!
//! Позднее переезжают в крейт `crates/core-shared` (воркспейс).

use serde::{Deserialize, Serialize};

/// Снимок `/api/health`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HealthResponse {
    /// `"ok"` или `"degraded"`.
    pub status: String,
    /// Версия приложения (`vX.Y.Z-<hash>`).
    pub version: String,
    /// Статусы коннекторов из реестра.
    #[serde(default)]
    pub connectors: Vec<ConnectorHealthDto>,
}

/// Статус одного коннектора в ответе health.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectorHealthDto {
    pub id: String,
    pub description: String,
    /// `"ok" | "degraded" | "down"`.
    pub status: String,
}
