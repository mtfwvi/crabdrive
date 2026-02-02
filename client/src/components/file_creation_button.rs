use crate::api::create_file;
use crate::components::file_selection_dialog::FileSelectionDialog;
use crate::model::node::{DecryptedNode, NodeMetadata};
use crate::utils::ui::format_number_as_ordinal;
use leptos::prelude::*;
use std::time::Duration;
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
    let add_toast = move |text: String, intent: ToastIntent| {
        toaster.dispatch_toast(
            move || {
                view! {
                    <Toast>
                        <ToastBody>{text}</ToastBody>
                    </Toast>
                }
            },
            ToastOptions::default()
                .with_intent(intent)
                .with_timeout(Duration::from_millis(10_000)),
        )
    };

    let file_selection_dialog_open = RwSignal::new(false);

    let creation_action = Action::new_local(move |input: &FileList| {
        let file_list = input.to_owned();
        let file_count = file_list.length();

        add_toast(
            format!("Uploading {} files...", file_count),
            ToastIntent::Info,
        );

        async move {
            let mut parent = parent_node.get();
            for i in 0..file_count {
                let file = file_list.get(i).unwrap();
                let result = create_file(&mut parent, file.name(), file).await;
                if result.is_err() {
                    return Err(format!(
                        "Error on the {} file: {}; aborted upload.",
                        format_number_as_ordinal(i + 1),
                        result.unwrap_err()
                    ));
                }
            }
            Ok(())
        }
    });

    Effect::new(move || {
        let status = creation_action.value().get();
        if status.is_some() {
            match status.unwrap() {
                Ok(_) => {
                    add_toast("Upload complete".to_string(), ToastIntent::Success);
                    on_created.run(())
                }
                Err(e) => {
                    add_toast(format!("Failed to create file: {}", e), ToastIntent::Error);
                    on_created.run(())
                }
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
        />
    }
}
