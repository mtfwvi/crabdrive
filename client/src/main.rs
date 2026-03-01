mod api;
mod components;
mod constants;
mod model;
mod pages;
mod theme;
mod utils;

use crate::theme::get_theme;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use leptos_use::use_preferred_dark;
use pages::home_page::HomePage;
use pages::login_page::LoginPage;
use thaw::{ConfigProvider, ToastPosition, ToasterProvider};
use tracing_subscriber::fmt::format::DefaultFields;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_web::{MakeWebConsoleWriter, performance_layer};

use crate::pages::accept_share_page::AcceptSharePage;
use crate::pages::home_page::HomePageType;
use crate::pages::login_page::LoginType;
#[cfg(test)]
use wasm_bindgen_test::wasm_bindgen_test_configure;

// instruct wasm-pack to run all test in the browser (otherwise node is used)
#[cfg(test)]
wasm_bindgen_test_configure!(run_in_browser);

fn main() {
    console_error_panic_hook::set_once();

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_target(false)
        .with_line_number(true)
        .without_time()
        .with_writer(MakeWebConsoleWriter::new());
    let perf_layer = performance_layer().with_details_from_fields(DefaultFields::default());

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init();

    // TODO: Test login!

    mount_to_body(CrabDrive)
}

#[component]
fn CrabDrive() -> impl IntoView {
    let theme = RwSignal::new(get_theme(false));

    let is_dark_preferred = use_preferred_dark();

    Effect::new(move || {
        let adaptive_theme = get_theme(is_dark_preferred.get());
        theme.set(adaptive_theme)
    });

    view! {
        <ConfigProvider theme>
            // Provide contrast against page background in dark theme
            // + provide space for "Dismiss all" button below toasts
            <style>
                ".thaw-toast { outline: 1px solid lightgray; } .thaw-toaster--bottom-start { bottom: 60px}"
            </style>
            <ToasterProvider position=ToastPosition::BottomStart>
                <Router>
                    <Routes fallback=|| "Frontend route not found">
                        <Route
                            path=path!("")
                            view=move || view! { <HomePage view_type=HomePageType::Folder /> }
                        />
                        <Route
                            path=path!("/shared")
                            view=move || view! { <HomePage view_type=HomePageType::Shared /> }
                        />
                        <Route
                            path=path!("/shared/:shareId")
                            view=move || view! { <AcceptSharePage /> }
                        />
                        <Route
                            path=path!("/trash")
                            view=move || view! { <HomePage view_type=HomePageType::Trash /> }
                        />
                        <Route
                            path=path!("/register")
                            view=move || view! { <LoginPage login_type=LoginType::Register /> }
                        />
                        <Route
                            path=path!("/login")
                            view=move || view! { <LoginPage login_type=LoginType::Login /> }
                        />
                        <Route
                            path=path!("/:id")
                            view=move || view! { <HomePage view_type=HomePageType::Folder /> }
                        />
                    </Routes>
                </Router>
            </ToasterProvider>
        </ConfigProvider>
    }
}
