pub mod app;
pub mod components;
pub mod functions;
pub mod pages;

cfg_if::cfg_if! {
    if #[cfg(any(feature="ssr", feature="backend"))] {
        pub mod api;
        pub mod database;
        pub mod errors;
        pub mod prelude;
        pub mod utils;
    }

}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
