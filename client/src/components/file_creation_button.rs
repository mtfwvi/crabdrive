use crate::api::create_file;
use crate::components::file_selection_dialog::FileSelectionDialog;
use crate::constants::{DEFAULT_TOAST_TIMEOUT, INFINITE_TOAST_TIMEOUT};
use crate::model::node::{DecryptedNode, NodeMetadata};
use crate::utils::ui::format_number_as_ordinal;
use crabdrive_common::uuid::UUID;
use leptos::prelude::*;
use thaw::{
    Button, ButtonAppearance, Spinner, SpinnerSize, Toast, ToastIntent, ToastOptions, ToastTitle,
    ToastTitleMedia, ToasterInjection,
};
use web_sys::File;

#[component]
pub fn FileCreationButton(
    #[prop(into)] parent_node: Signal<DecryptedNode>,
    on_created: Callback<()>,
) -> impl IntoView {
    let toaster = ToasterInjection::expect_context();
    let upload_in_progress_toast_id = UUID::random();
    let add_upload_in_progress_toast = move |file_count| {
        toaster.dispatch_toast(
            move || {
                view! {
                    <Toast>
                        <ToastTitle>
                            {move || {
                                format!(
                                    "Uploading {} file{}...",
                                    file_count,
                                    if file_count == 1 { "" } else { "s" },
                                )
                            }} <ToastTitleMedia slot>
                                <Spinner size=SpinnerSize::Tiny />
                            </ToastTitleMedia>
                        </ToastTitle>
                    </Toast>
                }
            },
            ToastOptions::default()
                .with_id(upload_in_progress_toast_id.into())
                .with_intent(ToastIntent::Info)
                .with_timeout(INFINITE_TOAST_TIMEOUT),
        )
    };
    let add_toast = move |text: String, intent: ToastIntent| {
        toaster.dispatch_toast(
            move || {
                view! {
                    <Toast>
                        <ToastTitle>{text}</ToastTitle>
                    </Toast>
                }
            },
            ToastOptions::default().with_intent(intent).with_timeout(
                if matches!(intent, ToastIntent::Error) {
                    INFINITE_TOAST_TIMEOUT
                } else {
                    DEFAULT_TOAST_TIMEOUT
                },
            ),
        )
    };

    let file_selection_dialog_open = RwSignal::new(false);

    let creation_action = Action::new_local(move |input: &Vec<File>| {
        let files = input.to_owned();
        let file_count = files.len();

        add_upload_in_progress_toast(file_count);

        async move {
            let mut parent = parent_node.get();
            for (i, file) in files.into_iter().enumerate() {
                let result = create_file(&mut parent, file.name(), file).await;
                if result.is_err() {
                    toaster.dismiss_toast(upload_in_progress_toast_id.into());
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
            toaster.dismiss_toast(upload_in_progress_toast_id.into());
            match status.unwrap() {
                Ok(_) => add_toast("Upload complete".to_string(), ToastIntent::Success),
                Err(e) => add_toast(format!("Failed to create file: {}", e), ToastIntent::Error),
            }
            on_created.run(())
        }
    });

    let on_files_selected = Callback::new(move |files: Vec<File>| {
        creation_action.dispatch_local(files);
        file_selection_dialog_open.set(false);
    });

    view! {
        <Button
            on_click=move |_| file_selection_dialog_open.set(true)
            appearance=ButtonAppearance::Primary
            icon=icondata_mdi::MdiPlus
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
