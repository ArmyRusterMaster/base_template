use tokio::signal;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use base_template::app::*;
    use base_template::config::AppConfig;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    // Загружаем типизированную конфигурацию (config/app.yaml + переменные APP_*)
    // и валидируем её на старте: любая ошибка → fail-fast, сервер не поднимается.
    let settings = AppConfig::load().unwrap_or_else(|err| {
        eprintln!("❌ Невалидная конфигурация: {err}");
        std::process::exit(1);
    });

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

    let app = Router::new()
        // SSR: на каждый маршрут отдаём `shell` (HTML-каркас + сердные скрипты).
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        // Статика из site-root (JS/WASM/CSS) и 404 → shell.
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    log!("{} запускается на http://{}", settings.app_name, addr);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Не удалось привязать TCP-сокет к адресу сервера");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Сервер завершился с ошибкой");
}

/// Ожидание сигналов завершения (Ctrl+C / SIGTERM) для graceful shutdown.
pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Не удалось установить обработчик Ctrl+C");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Не удалось установить обработчик SIGTERM")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => println!("🛑 Получен сигнал Ctrl+C, завершаем работу..."),
        _ = terminate => println!("🛑 Получен сигнал SIGTERM, завершаем работу..."),
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // Для клиентской стороны отдельная main не нужна:
    // гидратация выполняется из lib.rs (hydrate()).
}
