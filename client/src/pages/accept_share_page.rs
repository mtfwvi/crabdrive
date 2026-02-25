use crate::api::accept_share;
use crate::constants::DEFAULT_TOAST_TIMEOUT;
use crate::utils::browser::get_current_url;
use leptos::{IntoView, component, view};
use leptos_router::hooks::use_navigate;
use thaw::ToasterInjection;
use thaw::{Toast, ToastIntent, ToastOptions, ToastTitle};

#[component]
pub fn AcceptSharePage() -> impl IntoView {
    let navigate = use_navigate();

    // copied from home page
    let toaster = ToasterInjection::expect_context();
    let add_toast = move |text: String| {
        toaster.dispatch_toast(
            move || {
                view! {
                    <Toast>
                        <ToastTitle>{text}</ToastTitle>
                    </Toast>
                }
            },
            ToastOptions::default()
                .with_intent(ToastIntent::Error)
                .with_timeout(DEFAULT_TOAST_TIMEOUT),
        )
    };

    leptos::reactive::spawn_local(async move {
        let url = get_current_url().unwrap();
        let node_id = accept_share(&url).await;

        if let Err(err) = node_id {
            add_toast(err.to_string());
            return;
        }

        navigate(&format!("/{}", node_id.unwrap()), Default::default());
    });
}
