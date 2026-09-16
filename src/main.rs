//! Бинарник SSR-сервера (axum).
//!
//! Вся серверная часть компилируется только с feature `ssr`; без неё `main`
//! остаётся заглушкой, а гидратация запускается из `lib.rs::hydrate()`.

#[cfg(feature = "ssr")]
use std::sync::Arc;

#[cfg(feature = "ssr")]
use axum::extract::{FromRef, State};
#[cfg(feature = "ssr")]
use axum::routing::get;
#[cfg(feature = "ssr")]
use axum::{Json, Router};
#[cfg(feature = "ssr")]
use base_template::app::{shell, App};
#[cfg(feature = "ssr")]
use base_template::config::AppConfig;
#[cfg(feature = "ssr")]
use base_template::connectors::{ConnectorRegistry, ConnectorStatus};
#[cfg(feature = "ssr")]
use base_template::version::version;
#[cfg(feature = "ssr")]
use core_shared::{ConnectorHealthDto, HealthResponse};
#[cfg(feature = "ssr")]
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_axum::{generate_route_list, LeptosRoutes};
#[cfg(feature = "ssr")]
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};

/// Состояние приложения: опции Leptos + реестр коннекторов.
#[cfg(feature = "ssr")]
#[derive(Clone)]
struct AppState {
    leptos_options: LeptosOptions,
    connectors: Arc<ConnectorRegistry>,
}

// `LeptosRoutes` и `file_and_error_handler` из leptos_axum требуют, чтобы
// `LeptosOptions: FromRef<S>` для состояния роутера: так SSR-хендлеры получают
// опции Leptos из `AppState`.
#[cfg(feature = "ssr")]
impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    // Конфигурация (fail-fast) и логирование — первыми.
    let settings = AppConfig::load().unwrap_or_else(|err| {
        eprintln!("❌ Невалидная конфигурация: {err}");
        std::process::exit(1);
    });
    base_template::logging::init(&settings.log_level);

    let conf = get_configuration(None).expect("Не удалось загрузить конфигурацию Leptos");
    let mut leptos_options = conf.leptos_options;

    // Адрес для bind берём из нашей конфигурации, а не из env-переменных Leptos.
    let addr = settings.server_addr().unwrap_or_else(|err| {
        eprintln!("❌ Невалидный адрес сервера: {err}");
        std::process::exit(1);
    });
    leptos_options.site_addr = addr;

    // Маршруты, зарегистрированные в компоненте App (роутер Leptos).
    let routes = generate_route_list(App);

    let state = AppState {
        leptos_options,
        connectors: Arc::new(ConnectorRegistry::with_defaults()),
    };

    // `leptos_routes` принимает ссылку на состояние целиком (`&S`), а не на
    // `LeptosOptions` — опции при этом извлекаются через `FromRef<AppState>`.
    let shell_options = state.leptos_options.clone();

    let app = Router::new()
        // Инфраструктурный health-эндпоинт (статус + коннекторы).
        .route("/api/health", get(health_handler))
        // SSR: на каждый маршрут отдаём `shell` (HTML-каркас + hydration-скрипты).
        .leptos_routes(&state, routes, move || shell(shell_options.clone()))
        // Статика из site-root (JS/WASM/CSS) и 404 → shell.
        // Тип состояния указан явно: иначе вывод `S` неоднозначен
        // (FromRef<AppState> и рефлексивный FromRef<LeptosOptions>).
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        // request-id: на каждый запрос ставится x-request-id и возвращается в ответе.
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .with_state(state);

    tracing::info!(
        "{} v{} слушает http://{}",
        settings.app_name,
        version(),
        addr
    );
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Не удалось привязать TCP-сокет к адресу сервера");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Сервер завершился с ошибкой");
}

/// GET /api/health: версия приложения + агрегированный статус коннекторов.
#[cfg(feature = "ssr")]
async fn health_handler(State(state): State<AppState>) -> Json<HealthResponse> {
    let connectors = state.connectors.health().await;
    let all_ok = connectors.iter().all(|c| c.status == ConnectorStatus::Ok);

    Json(HealthResponse {
        status: if all_ok { "ok" } else { "degraded" }.to_owned(),
        version: version().to_owned(),
        connectors: connectors
            .into_iter()
            .map(|c| ConnectorHealthDto {
                id: c.id,
                description: c.description,
                status: c.status.as_str().to_owned(),
            })
            .collect(),
    })
}

/// Ожидание сигналов завершения (Ctrl+C / SIGTERM) для graceful shutdown.
#[cfg(feature = "ssr")]
pub async fn shutdown_signal() {
    tokio::select! {
        _ = ctrl_c() => tracing::info!("Получен сигнал Ctrl+C, завершаем работу..."),
        _ = terminate() => tracing::info!("Получен сигнал SIGTERM, завершаем работу..."),
    }
}

#[cfg(feature = "ssr")]
async fn ctrl_c() {
    tokio::signal::ctrl_c()
        .await
        .expect("Не удалось установить обработчик Ctrl+C");
}

#[cfg(feature = "ssr")]
#[cfg(unix)]
async fn terminate() {
    tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .expect("Не удалось установить обработчик SIGTERM")
        .recv()
        .await;
}

#[cfg(feature = "ssr")]
#[cfg(not(unix))]
async fn terminate() {
    std::future::pending::<()>().await;
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // Для клиентской стороны отдельная main не нужна:
    // гидратация выполняется из lib.rs (hydrate()).
}
