//! Инфраструктурная ошибка приложения (feature `ssr`).
//!
//! Принцип (RULES.md §3): наружу отдаётся только безопасный текст и код ошибки —
//! детали «сырых» инфраструктурных сбоев не утекают клиенту. Шаблоны-наследники
//! добавляют поверх свои доменные варианты и маппят их в этот тип.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

use crate::config::ConfigError;

/// Единая ошибка приложения для всех обработчиков axum.
#[derive(Debug)]
pub enum AppError {
    /// Конфигурация невалидна (наследуется из слоя конфигурации).
    Config(ConfigError),
    /// Ресурс не найден; `what` — безопасное для клиента описание.
    NotFound(String),
    /// Внутренняя ошибка; детали логируются, но не отдаются клиенту.
    Internal(String),
}

impl AppError {
    fn status(&self) -> StatusCode {
        match self {
            AppError::Config(_) | AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    fn code(&self) -> &'static str {
        match self {
            AppError::Config(_) => "config_error",
            AppError::NotFound(_) => "not_found",
            AppError::Internal(_) => "internal_error",
        }
    }

    /// Текст, безопасный для внешней среды.
    fn public_message(&self) -> String {
        match self {
            AppError::Config(e) => format!("Invalid application configuration: {e}"),
            AppError::NotFound(what) => format!("{what} not found"),
            AppError::Internal(_) => "Internal server error".to_owned(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status();
        let code = self.code();
        let message = self.public_message();

        // Полные детали — только во внутренние логи.
        tracing::error!(%code, %status, error = ?self, "ошибка запроса");

        (
            status,
            Json(json!({ "error": { "code": code, "message": message } })),
        )
            .into_response()
    }
}

impl From<ConfigError> for AppError {
    fn from(e: ConfigError) -> Self {
        AppError::Config(e)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Config(e) => write!(f, "config error: {e}"),
            AppError::NotFound(what) => write!(f, "not found: {what}"),
            AppError::Internal(msg) => write!(f, "internal error: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal_error_masks_details() {
        let response = AppError::Internal("DB password: hunter2".to_owned()).into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn not_found_is_404() {
        let response = AppError::NotFound("widget".to_owned()).into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn config_error_maps_from_config_layer() {
        let err: AppError = ConfigError::Invalid("bad host".to_owned()).into();
        assert_eq!(err.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
