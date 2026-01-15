pub(crate) mod components;
mod display_utils;
pub(crate) mod pages;
mod theme;

use crate::theme::get_theme;
use leptos::prelude::*;
use pages::home_page::HomePage;
use thaw::{ConfigProvider, ToastPosition, ToasterProvider};
use tracing_subscriber::fmt::format::DefaultFields;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_web::{MakeWebConsoleWriter, performance_layer};

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

    let theme = RwSignal::new(get_theme());

    mount_to_body(move || {
        view! {
            <ConfigProvider theme>
                <ToasterProvider position=ToastPosition::BottomStart>
                    <HomePage />
                </ToasterProvider>
            </ConfigProvider>
        }
    })
}
