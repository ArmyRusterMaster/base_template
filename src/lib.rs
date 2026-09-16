// Leptos hydration generates deeply nested view/future types. The WASM release
// build exceeds rustc's default recursion limit (see CI query-depth overflow).
#![recursion_limit = "256"]

pub mod app;
pub mod components;
pub mod config;
pub mod version;

#[cfg(feature = "ssr")]
pub mod connectors;
#[cfg(feature = "ssr")]
pub mod error;
#[cfg(feature = "ssr")]
pub mod logging;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
