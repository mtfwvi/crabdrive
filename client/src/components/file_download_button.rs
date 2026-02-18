use crate::api::download_file;
use crate::constants::DEFAULT_TOAST_TIMEOUT;
use crate::model::node::DecryptedNode;
use crate::model::node::NodeMetadata;
use leptos::prelude::*;
use thaw::{
    Button, ButtonAppearance, Toast, ToastIntent, ToastOptions, ToastTitle, ToasterInjection,
};

#[component]
pub(crate) fn FileDownloadButton(#[prop(into)] node: Signal<DecryptedNode>) -> impl IntoView {
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

    let file_name = Signal::derive(move || {
        let NodeMetadata::V1(metadata) = node.get().metadata;
        metadata.name
    });

    let download_action = Action::new_local(|input: &DecryptedNode| {
        let node = input.to_owned();
        async move { download_file(node).await.map_err(|err| err.to_string()) }
    });
    let handle_download = move |_| {
        download_action.dispatch(node.get().clone());
    };

    Effect::new(move || {
        let status = download_action.value().get();
        if status.is_some() {
            let response = status.unwrap();
            if response.is_err() {
                add_toast(format!(
                    "Failed to download {}: {}",
                    file_name.get(),
                    response.err().unwrap()
                ))
            }
        }
    });

    view! {
        <Button
            on_click=handle_download
            appearance=ButtonAppearance::Primary
            icon=icondata_mdi::MdiDownload
            block=true
        >
            "Download"
        </Button>
    }
}
