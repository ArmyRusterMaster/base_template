//! UI-скелет: шапка, сайд-меню, футер, layout.
//!
//! Стили — утилитарные классы Tailwind (RULES.md §4).

use leptos::prelude::*;
use leptos_router::components::A;

use crate::version::version;

/// Корневой layout: шапка + (сайд-меню + контент) + футер.
#[component]
pub fn Layout(children: Children) -> impl IntoView {
    let sidebar_open = RwSignal::new(false);

    view! {
        <div class="flex min-h-screen flex-col bg-gray-950 text-gray-100">
            <Header sidebar_open/>
            <div class="flex flex-1">
                <Sidebar sidebar_open/>
                <main class="min-w-0 flex-1 px-4 py-6 md:px-8">{children()}</main>
            </div>
            <Footer/>
        </div>
    }
}

/// Шапка: логотип + версия + верхняя навигация + кнопка сайд-меню (мобильные).
#[component]
pub fn Header(sidebar_open: RwSignal<bool>) -> impl IntoView {
    view! {
        <header class="sticky top-0 z-20 flex h-14 items-center justify-between border-b border-gray-800 bg-gray-950/90 px-4 backdrop-blur">
            <div class="flex items-center gap-3">
                <button
                    class="rounded-md p-2 text-gray-400 hover:bg-gray-800 hover:text-gray-100 md:hidden"
                    on:click=move |_| sidebar_open.update(|open| *open = !*open)
                >
                    <svg
                        class="h-5 w-5"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                        stroke-width="2"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M4 6h16M4 12h16M4 18h16"
                        />
                    </svg>
                </button>
                <A href="/" class="text-lg font-semibold text-gray-100">
                    "base_template"
                </A>
                <span class="hidden rounded-full border border-gray-700 px-2 py-0.5 text-xs text-gray-400 sm:inline">
                    {version()}
                </span>
            </div>
            <nav class="hidden items-center gap-4 text-sm md:flex">
                <A href="/" class="text-gray-400 hover:text-gray-100">
                    "Главная"
                </A>
                <A href="/status" class="text-gray-400 hover:text-gray-100">
                    "Статус"
                </A>
                <A href="/account" class="text-gray-400 hover:text-gray-100">
                    "Кабинет"
                </A>
            </nav>
        </header>
    }
}

/// Боковое меню: на десктопе постоянно, на мобильных — по кнопке в шапке.
#[component]
pub fn Sidebar(sidebar_open: RwSignal<bool>) -> impl IntoView {
    let is_desktop = leptos_use::use_media_query("(min-width: 768px)");

    view! {
        <Show when=move || is_desktop.get() || sidebar_open.get()>
            <aside class="w-56 shrink-0 border-r border-gray-800 bg-gray-950">
                <nav class="flex flex-col gap-1 p-3 text-sm">
                    <SidebarLink href="/" label="Главная"/>
                    <SidebarLink href="/account" label="Личный кабинет"/>
                    <SidebarLink href="/status" label="Статус сервисов"/>
                </nav>
            </aside>
        </Show>
    }
}

#[component]
fn SidebarLink(href: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <A href=href class="rounded-lg px-3 py-2 text-gray-400 hover:bg-gray-800 hover:text-gray-100">
            {label}
        </A>
    }
}

/// Футер: copyright + версия сборки.
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t border-gray-800 px-4 py-4 text-center text-xs text-gray-500">
            "© base_template · " {version()} " · Rust + Leptos + Axum"
        </footer>
    }
}
