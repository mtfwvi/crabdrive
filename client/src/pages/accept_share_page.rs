use crate::api::{accept_share, download_file, get_accepted_nodes};
use crate::constants::DEFAULT_TOAST_TIMEOUT;
use crate::utils::browser::get_current_url;
use crabdrive_common::storage::NodeType;
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
        let node_id = node_id.unwrap();

        let accepted_nodes = get_accepted_nodes().await;
        if accepted_nodes.is_err() {
            return;
        }

        let accepted_nodes = accepted_nodes.unwrap();
        let current_node = accepted_nodes.into_iter().find(|node| node.id == node_id);

        if current_node.is_none() {
            return;
        }
        let current_node = current_node.unwrap();

        if current_node.node_type == NodeType::File {
            let download_result = download_file(current_node).await;
            if let Err(err) = download_result {
                add_toast(format!("Failed to download shared file: {}", err));
            }
        } else {
            navigate(&format!("/{}", node_id), Default::default());
        }
    });
}
