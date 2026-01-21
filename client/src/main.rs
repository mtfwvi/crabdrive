pub(crate) mod api;
pub(crate) mod components;
pub(crate) mod constants;
pub(crate) mod model;
pub(crate) mod pages;
pub(crate) mod utils;

use leptos::prelude::*;
use pages::demo_page::DemoPage;
use tracing_subscriber::fmt::format::DefaultFields;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_web::{MakeWebConsoleWriter, performance_layer};

#[cfg(test)]
use wasm_bindgen_test::wasm_bindgen_test_configure;

#[cfg(test)]
wasm_bindgen_test_configure!(run_in_browser);

fn main() {
    console_error_panic_hook::set_once();

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .without_time()
        .with_writer(MakeWebConsoleWriter::new());
    let perf_layer = performance_layer().with_details_from_fields(DefaultFields::default());

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init();

    mount_to_body(DemoPage)
}
