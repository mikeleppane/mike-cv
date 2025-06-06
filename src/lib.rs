pub mod app;
pub mod components;
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::App;

    use console_error_panic_hook;

    console_error_panic_hook::set_once();

    leptos::mount::hydrate_body(App);
}
