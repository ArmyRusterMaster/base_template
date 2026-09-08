use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, StaticSegment},
    A,
};

use crate::components::Layout;
use crate::dto::HealthResponse;
use crate::version::version;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="ru">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

/// Корневой компонент: layout + страницы.
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/base_template.css"/>
        <Title text="base_template"/>
        <Router>
            <Layout>
                <Routes fallback=|| view! { <NotFound/> }>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("account") view=AccountPage/>
                    <Route path=StaticSegment("status") view=StatusPage/>
                </Routes>
            </Layout>
        </Router>
    }
}

// ---------------------------------------------------------------------------
// Страницы
// ---------------------------------------------------------------------------

/// `/` — приветствие и обзор возможностей шаблона.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <section class="mx-auto max-w-3xl">
            <h1 class="text-3xl font-bold">"Добро пожаловать в base_template"</h1>
            <p class="mt-3 text-gray-400">
                "Базовый фуллстек-шаблон (Rust + Leptos + Axum): UI-скелет, коннекторы,"
                " типизированная конфигурация и деплой уже настроены. Наследники"
                " добавляют бизнес-слой (auth, оплата, интеграции)."
            </p>
            <div class="mt-6 grid gap-4 sm:grid-cols-3">
                <FeatureCard
                    title="SSR + Hydration"
                    text="Сервер рендерит страницы, клиент принимает WASM-бандл с кеш-бастингом."
                />
                <FeatureCard
                    title="Коннекторы"
                    text="Трейт Connector + реестр: внешние интеграции подключаются одним файлом."
                />
                <FeatureCard
                    title="Deploy"
                    text="Docker (Ubuntu → Alpine, musl-статик), docker-compose и CI-конвейер."
                />
            </div>
        </section>
    }
}

#[component]
fn FeatureCard(title: &'static str, text: &'static str) -> impl IntoView {
    view! {
        <div class="rounded-xl border border-gray-800 bg-gray-900 p-4">
            <h3 class="font-semibold text-teal-400">{title}</h3>
            <p class="mt-1 text-sm text-gray-400">{text}</p>
        </div>
    }
}

/// `/account` — личный кабинет: профиль-заглушка + подключения + настройки.
#[component]
fn AccountPage() -> impl IntoView {
    view! {
        <section class="mx-auto max-w-3xl space-y-6">
            <h1 class="text-2xl font-bold">"Личный кабинет"</h1>

            <div class="rounded-xl border border-gray-800 bg-gray-900 p-6">
                <div class="flex items-center gap-4">
                    <div class="flex h-14 w-14 items-center justify-center rounded-full bg-teal-600 text-xl font-bold text-white">
                        "?"
                    </div>
                    <div>
                        <p class="font-semibold">"Пользователь"</p>
                        <p class="text-sm text-gray-400">"user@example.com"</p>
                        <p class="mt-1 text-xs text-gray-500">
                            "Заглушка: аутентификация добавляется в шаблоне микросааса."
                        </p>
                    </div>
                </div>
            </div>

            <div class="rounded-xl border border-gray-800 bg-gray-900 p-6">
                <h2 class="font-semibold">"Подключения"</h2>
                <p class="mt-1 text-sm text-gray-400">
                    "Коннекторы из реестра (данные /api/health). Новая интеграция —"
                    " один файл с реализацией Connector."
                </p>
                <ConnectorsList/>
            </div>

            <div class="rounded-xl border border-dashed border-gray-700 p-6 text-sm text-gray-500">
                "Настройки: секция-заглушка — расширяется в производных шаблонах."
            </div>
        </section>
    }
}

/// `/status` — статусы коннекторов и версия приложения.
#[component]
fn StatusPage() -> impl IntoView {
    view! {
        <section class="mx-auto max-w-3xl space-y-4">
            <h1 class="text-2xl font-bold">"Статус сервисов"</h1>
            <p class="text-sm text-gray-400">
                "Версия: " {version()} " · агрегированный статус из /api/health"
            </p>
            <ConnectorsList/>
        </section>
    }
}

/// 404 внутри layout.
#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center py-24 text-center">
            <p class="text-7xl font-bold text-teal-400">"404"</p>
            <p class="mt-4 text-lg text-gray-400">"Страница не найдена"</p>
            <A
                href="/"
                class="mt-6 rounded-lg bg-teal-600 px-4 py-2 text-sm font-medium text-white hover:bg-teal-500"
            >
                "На главную"
            </A>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Общие блоки
// ---------------------------------------------------------------------------

/// Список коннекторов: клиентский запрос /api/health + ручное обновление.
#[component]
fn ConnectorsList() -> impl IntoView {
    let health = RwSignal::new(None::<HealthResponse>);
    let refresh = RwSignal::new(0u32);

    // Эффекты выполняются только на клиенте — SSR отдаёт заглушку.
    Effect::new(move |_| {
        refresh.track();
        spawn(async move { health.set(fetch_health().await) });
    });

    view! {
        <div class="mt-4 space-y-2">
            <Show when=move || health.get().is_some()>
                <For
                    each=move || health.get().map(|h| h.connectors).unwrap_or_default()
                    key=|c| c.id.clone()
                    children=move |c| {
                        let status_class = match c.status.as_str() {
                            "ok" => "bg-emerald-500/10 text-emerald-400",
                            "degraded" => "bg-amber-500/10 text-amber-400",
                            _ => "bg-red-500/10 text-red-400",
                        };
                        view! {
                            <div class="flex items-center justify-between rounded-lg border border-gray-800 bg-gray-950 px-4 py-2">
                                <div>
                                    <p class="font-medium">{c.id.clone()}</p>
                                    <p class="text-xs text-gray-500">{c.description.clone()}</p>
                                </div>
                                <span class={format!("rounded-full px-2 py-0.5 text-xs {status_class}")}>
                                    {c.status.clone()}
                                </span>
                            </div>
                        }
                    }
                />
            </Show>
            <button
                class="rounded-md px-3 py-1.5 text-xs text-gray-400 hover:bg-gray-800 hover:text-gray-100"
                on:click=move |_| refresh.update(|v| *v += 1)
            >
                "Обновить"
            </button>
        </div>
    }
}

/// GET /api/health с клиента (только wasm; на SSR отдаётся пустая заглушка).
#[cfg(target_arch = "wasm32")]
async fn fetch_health() -> Option<HealthResponse> {
    use gloo_net::http::Request;
    Request::get("/api/health")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()
}

#[cfg(not(target_arch = "wasm32"))]
async fn fetch_health() -> Option<HealthResponse> {
    None // SSR: данные подгружаются на клиенте после гидратации
}
