//! Инициализация трейсинга (feature `ssr`).
//!
//! Приоритет уровня логов: `RUST_LOG` → `log_level` из конфигурации → `info`.

use tracing_subscriber::EnvFilter;

/// Инициализирует глобальный подписчик. Вызывать один раз в начале `main`.
pub fn init(level: &str) {
    let filter = std::env::var("RUST_LOG")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| level.to_owned());
    let filter = EnvFilter::try_new(&filter).unwrap_or_else(|_| EnvFilter::new("info"));

    // try_init: безопасно при повторном вызове (например, в тестах).
    let _ = tracing_subscriber::fmt().with_env_filter(filter.clone()).try_init();
    tracing::debug!("Логирование инициализировано (уровень: {filter})");
}