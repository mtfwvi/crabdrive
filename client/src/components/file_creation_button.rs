use crate::api::create_file;
use crate::components::file_selection_dialog::FileSelectionDialog;
use crate::model::node::{DecryptedNode, NodeMetadata};
use leptos::prelude::*;
use thaw::{
    Button, ButtonAppearance, Toast, ToastBody, ToastIntent, ToastOptions, ToasterInjection,
};
use web_sys::FileList;

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

    let creation_action = Action::new_local(move |input: &FileList| {
        let file_list = input.to_owned();
        if file_list.length() == 0 {
            add_toast(String::from("No file selected"));
        }

        async move {
            let file = file_list.get(0).unwrap();
            create_file(parent_node.get(), file.name(), file).await
        }
    });

    Effect::new(move || {
        let status = creation_action.value().get();
        if status.is_some() {
            match status.unwrap() {
                Ok(_) => on_created.run(()),
                Err(e) => add_toast(format!("Failed to create file: {}", e)),
            }
        }
    });

    let on_files_selected = Callback::new(move |file_list: FileList| {
        creation_action.dispatch_local(file_list);
        file_selection_dialog_open.set(false);
    });

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
            on_confirm=on_files_selected
            title=Signal::derive(move || {
                let name = Signal::derive(move || {
                    let NodeMetadata::V1(metadata) = parent_node.get().metadata;
                    metadata.name
                });
                format!("Upload files to {}", name.get())
            })
            allow_multiple=false
        />
    }
}
