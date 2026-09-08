//! Типизированная конфигурация приложения: файл `config/app.yaml` + переменные окружения `APP_*`.
//!
//! Приоритет источников (от слабого к сильному):
//! 1. дефолты из `AppConfig::default()`;
//! 2. значения из `config/app.yaml` (отсутствующие поля добираются дефолтами);
//! 3. переменные окружения `APP_APP_NAME`, `APP_SERVER_HOST`, `APP_SERVER_PORT`.
//!
//! Валидация:
//! * типы проверяются компилятором (`serde::Deserialize` + строгая производная структура);
//! * значения проверяются на старте методом [`AppConfig::validate`] — fail-fast: при любой
//!   ошибке сервер не поднимается, а слой конфигурации сразу падает с понятным сообщением.

use serde::Deserialize;
use std::net::{IpAddr, SocketAddr};
use std::path::Path;

/// Путь к yaml-источнику конфигурации (относительно корня проекта).
pub const CONFIG_FILE: &str = "config/app.yaml";

/// Результат операций конфигурации.
pub type Result<T> = std::result::Result<T, ConfigError>;

/// Ошибки загрузки и валидации конфигурации.
#[derive(Debug)]
pub enum ConfigError {
    /// Ошибка чтения файла `config/app.yaml`.
    Read(std::io::Error),
    /// Ошибка разбора YAML.
    Parse(serde_yaml::Error),
    /// Ошибка разбора переменной окружения.
    Env(String),
    /// Значение не прошло валидацию (например, невалидный IP).
    Invalid(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Read(e) => write!(f, "невозможно прочитать {CONFIG_FILE}: {e}"),
            ConfigError::Parse(e) => write!(f, "ошибка разбора {CONFIG_FILE}: {e}"),
            ConfigError::Env(e) => write!(f, "ошибка переменной окружения: {e}"),
            ConfigError::Invalid(e) => write!(f, "невалидное значение: {e}"),
        }
    }
}

impl std::error::Error for ConfigError {}

/// Корневая конфигурация приложения.
#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct AppConfig {
    /// Имя приложения (используется в логах и авторелоаде).
    pub app_name: String,
    /// Конфигурация HTTP-сервера.
    pub server: ServerConfig,
}

/// Конфигурация HTTP-сервера.
#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ServerConfig {
    /// IP-адрес для слушания (например, `127.0.0.1` локально, `0.0.0.0` в Docker).
    pub host: String,
    /// Порт HTTP-сервера.
    pub port: u16,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app_name: "base_template".to_owned(),
            server: ServerConfig::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_owned(),
            port: 3000,
        }
    }
}
impl AppConfig {
    /// Загружает конфигурацию: `config/app.yaml` + переменные `APP_*`, затем валидирует.
    pub fn load() -> Result<Self> {
        let config = Self::from_yaml_file(Path::new(CONFIG_FILE))?;
        config.apply_env()?.validate_and_return()
    }

    /// Читает файл конфигурации; если файла нет — берёт дефолты.
    fn from_yaml_file(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = std::fs::read_to_string(path).map_err(ConfigError::Read)?;
        serde_yaml::from_str(&raw).map_err(ConfigError::Parse)
    }

    /// Накладывает переменные окружения `APP_*` поверх значений из файла.
    fn apply_env(mut self) -> Result<Self> {
        if let Ok(v) = std::env::var("APP_APP_NAME") {
            self.app_name = v;
        }
        if let Ok(v) = std::env::var("APP_SERVER_HOST") {
            self.server.host = v;
        }
        if let Ok(v) = std::env::var("APP_SERVER_PORT") {
            self.server.port = v
                .parse::<u16>()
                .map_err(|e| ConfigError::Env(format!("APP_SERVER_PORT: {e}")))?;
        }
        Ok(self)
    }

    /// Валидирует значения и возвращает конфигурацию (fail-fast на старте).
    pub fn validate_and_return(self) -> Result<Self> {
        let _ = self.server_addr()?;
        Ok(self)
    }

    /// Валидирует значения (без потребления `self`).
    pub fn validate(&self) -> Result<()> {
        let _ = self.server_addr()?;
        Ok(())
    }

    /// Адрес для `axum::serve`, собранный из `server.host` + `server.port`.
    pub fn server_addr(&self) -> Result<SocketAddr> {
        self.server.addr()
    }
}

impl ServerConfig {
    /// Парсит IP-адрес и порт в `SocketAddr` с понятной ошибкой, если значение невалидно.
    pub fn addr(&self) -> Result<SocketAddr> {
        let ip: IpAddr = self.host.parse().map_err(|_| {
            ConfigError::Invalid(format!(
                "server.host `{}` не является валидным IP-адресом",
                self.host
            ))
        })?;
        if self.port == 0 {
            return Err(ConfigError::Invalid(
                "server.port не может быть равен 0".to_owned(),
            ));
        }
        Ok(SocketAddr::new(ip, self.port))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_valid() {
        let config = AppConfig::default();
        assert!(config.validate().is_ok());
        assert_eq!(config.server.addr().unwrap().port(), 3000);
    }

    #[test]
    fn parses_yaml_with_defaults_for_missing_fields() {
        let config: AppConfig =
            serde_yaml::from_str("app_name: my-app\nserver:\n  port: 8080\n").unwrap();
        assert_eq!(config.app_name, "my-app");
        // host не указан — подставится дефолт
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 8080);
    }

    #[test]
    fn rejects_unknown_keys() {
        let result: std::result::Result<AppConfig, _> =
            serde_yaml::from_str("app_name: x\nunknown_key: 1\n");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_host() {
        let mut config = AppConfig::default();
        config.server.host = "not-an-ip".to_owned();
        assert!(config.validate().is_err());
    }
}
