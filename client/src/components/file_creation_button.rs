use crate::components::file_selection_dialog::FileSelectionDialog;
use crate::model::node::{DecryptedNode, NodeMetadata};
use leptos::prelude::*;
use thaw::{
    Button, ButtonAppearance, Toast, ToastBody, ToastIntent, ToastOptions, ToasterInjection,
};

#[component]
pub(crate) fn FileCreationButton(
    #[prop(into)] parent_node: Signal<DecryptedNode>,
    on_created: Callback<()>,
) -> impl IntoView {
    let toaster = ToasterInjection::expect_context();
    let add_toast = move |text: String| {
        toaster.dispatch_toast(
            move || {
                view! {
                    <Toast>
                        <ToastBody>{text}</ToastBody>
                    </Toast>
                }
            },
            ToastOptions::default().with_intent(ToastIntent::Info),
        )
    };

    let file_selection_dialog_open = RwSignal::new(false);

    view! {
        <Button
            on_click=move |_| file_selection_dialog_open.set(true)
            appearance=ButtonAppearance::Primary
            icon=icondata::MdiPlus
        >
            "Upload file"
        </Button>

        <FileSelectionDialog
            open=file_selection_dialog_open
            on_confirm=move |file_list| {
                add_toast(format!("Received {} file(s) to upload", file_list.length()));
                file_selection_dialog_open.set(false);
                on_created.run(());
            }
            title=Signal::derive(move || {
                let name = Signal::derive(move || {
                    let NodeMetadata::V1(metadata) = parent_node.get().metadata;
                    metadata.name
                });
                format!("Upload files to {}", name.get())
            })
        />
    }
}
