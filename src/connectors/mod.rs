//! Каркас внешних коннекторов (feature `ssr`).
//!
//! Базовый шаблон определяет **только контракт**: трейт [`Connector`] и
//! реестр [`ConnectorRegistry`]. Конкретные интеграции (оплата, Telegram,
//! почта и т.д.) добавляются в шаблонах-наследниках: один файл с реализацией
//! трейта + одна строка `registry.register(...)`.

use async_trait::async_trait;

/// Состояние здоровья коннектора.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectorStatus {
    Ok,
    Degraded,
    Down,
}

impl ConnectorStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConnectorStatus::Ok => "ok",
            ConnectorStatus::Degraded => "degraded",
            ConnectorStatus::Down => "down",
        }
    }
}

/// Трейт внешнего коннектора (интеграции со сторонним сервисом).
///
/// База определяет минимальный каркас; наследники расширяют его своими
/// трейтами поверх (авторизация, отправка, подписки и т.д.).
#[async_trait]
pub trait Connector: Send + Sync {
    /// Уникальный идентификатор (например, `"stripe"`, `"telegram"`).
    fn id(&self) -> &'static str;

    /// Короткое человекочитаемое описание.
    fn description(&self) -> &'static str;

    /// Проверка доступности внешнего сервиса.
    async fn health(&self) -> ConnectorStatus;
}

/// Снимок статуса одного коннектора (отдаётся в `/api/health`).
#[derive(Debug, Clone, serde::Serialize)]
pub struct ConnectorHealth {
    pub id: String,
    pub description: String,
    pub status: ConnectorStatus,
}

/// Реестр подключённых коннекторов.
#[derive(Default)]
pub struct ConnectorRegistry {
    connectors: Vec<Box<dyn Connector>>,
}

impl ConnectorRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Реестр с базовым демо-коннектором.
    pub fn with_defaults() -> Self {
        let mut registry = Self::default();
        registry.register(Box::new(EchoConnector));
        registry
    }

    /// Регистрация коннектора (вызывается при инициализации приложения).
    pub fn register(&mut self, connector: Box<dyn Connector>) {
        self.connectors.push(connector);
    }

    pub fn len(&self) -> usize {
        self.connectors.len()
    }

    pub fn is_empty(&self) -> bool {
        self.connectors.is_empty()
    }

    /// Агрегированный статус всех коннекторов.
    pub async fn health(&self) -> Vec<ConnectorHealth> {
        let mut out = Vec::with_capacity(self.connectors.len());
        for connector in &self.connectors {
            out.push(ConnectorHealth {
                id: connector.id().to_owned(),
                description: connector.description().to_owned(),
                status: connector.health().await,
            });
        }
        out
    }
}

/// Демо-коннектор-заглушка: образец реализации для наследников.
pub struct EchoConnector;

#[async_trait]
impl Connector for EchoConnector {
    fn id(&self) -> &'static str {
        "echo"
    }

    fn description(&self) -> &'static str {
        "Demo no-op connector (base template sample)"
    }

    async fn health(&self) -> ConnectorStatus {
        ConnectorStatus::Ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn registry_reports_health_of_registered_connectors() {
        let mut registry = ConnectorRegistry::new();
        registry.register(Box::new(EchoConnector));
        assert_eq!(registry.len(), 1);

        let health = registry.health().await;
        assert_eq!(health.len(), 1);
        assert_eq!(health[0].id, "echo");
        assert_eq!(health[0].status, ConnectorStatus::Ok);
    }

    #[test]
    fn status_serializes_snake_case() {
        assert_eq!(
            serde_json::to_string(&ConnectorStatus::Degraded).unwrap(),
            "\"degraded\""
        );
    }
}
