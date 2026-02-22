use crate::api::create_folder;
use crate::components::input_dialog::InputDialog;
use crate::constants::INFINITE_TOAST_TIMEOUT;
use crate::model::node::DecryptedNode;
use leptos::prelude::*;
use thaw::{Button, Toast, ToastIntent, ToastOptions, ToastTitle, ToasterInjection};

#[component]
pub(crate) fn FolderCreationButton(
    #[prop(into)] parent_node: Signal<DecryptedNode>,
    on_created: Callback<()>,
) -> impl IntoView {
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
                .with_timeout(INFINITE_TOAST_TIMEOUT),
        )
    };

    let folder_creation_dialog_open = RwSignal::new(false);

    let creation_action = Action::new_local(move |input: &String| {
        let name = input.to_owned();
        async move {
            create_folder(parent_node.get(), name)
                .await
                .map_err(|err| err.to_string())
        }
    });

    Effect::new(move || {
        let status = creation_action.value().get();
        if status.is_some() {
            match status.unwrap() {
                Ok(_) => on_created.run(()),
                Err(e) => add_toast(format!("Failed to create folder: {}", e)),
            }
        }
    });

    let on_name_confirmed = Callback::new(move |name: String| {
        creation_action.dispatch(name);
        folder_creation_dialog_open.set(false);
    });

    view! {
        <Button
            on_click=move |_| folder_creation_dialog_open.set(true)
            icon=icondata_mdi::MdiFolderPlusOutline
        >
            "Create folder"
        </Button>

        <InputDialog
            open=folder_creation_dialog_open
            title="Create new folder"
            placeholder="Folder name"
            confirm_label="Create"
            on_confirm=on_name_confirmed
        />
    }
}
