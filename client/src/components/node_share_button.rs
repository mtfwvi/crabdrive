use crate::api::share_node;
use crate::constants::INFINITE_TOAST_TIMEOUT;
use crate::model::node::DecryptedNode;
use crate::model::node::NodeMetadata;
use leptos::prelude::*;
use leptos_use::{UseClipboardReturn, use_clipboard};
use thaw::{
    Button, ButtonAppearance, Toast, ToastIntent, ToastOptions, ToastTitle, ToasterInjection,
};

#[component]
pub(crate) fn NodeShareButton(#[prop(into)] node: Signal<DecryptedNode>) -> impl IntoView {
    let UseClipboardReturn { copy, .. } = use_clipboard();
    let toaster = ToasterInjection::expect_context();

    let add_toast = move |text: String, intent: ToastIntent| {
        toaster.dispatch_toast(
            move || {
                view! {
                    <Toast>
                        <ToastTitle>{text}</ToastTitle>
                    </Toast>
                }
            },
            ToastOptions::default()
                .with_intent(intent)
                .with_timeout(INFINITE_TOAST_TIMEOUT),
        )
    };

    let file_name = Signal::derive(move || {
        let NodeMetadata::V1(metadata) = node.get().metadata;
        metadata.name
    });

    let create_link_action = Action::new_local(|input: &DecryptedNode| {
        let node = input.to_owned();
        async move { share_node(&node).await.map_err(|err| err.to_string()) }
    });
    let handle_share = move |_| {
        create_link_action.dispatch(node.get().clone());
    };

    Effect::new(move || {
        let status = create_link_action.value().get();
        if status.is_some() {
            match status.unwrap() {
                Ok(url) => {
                    copy(&url);
                    add_toast(
                        format!(
                            "Share this link with the recipient (also copied to clipboard): {}",
                            url
                        ),
                        ToastIntent::Success,
                    )
                }
                Err(e) => add_toast(
                    format!(
                        "Failed to create sharing link for {}: {}",
                        file_name.get(),
                        e
                    ),
                    ToastIntent::Error,
                ),
            }
        }
    });

    view! {
        <Button
            on_click=handle_share
            appearance=ButtonAppearance::Secondary
            icon=icondata_mdi::MdiShare
            block=true
        >
            "Share"
        </Button>
    }
}
