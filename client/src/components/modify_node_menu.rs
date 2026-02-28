use crate::api::{move_node, move_node_to_trash, rename_node};
use crate::components::basic::folder_selection_dialog::FolderSelectionDialog;
use crate::components::basic::input_dialog::InputDialog;
use crate::components::file_selection_dialog::FileSelectionDialog;
use crate::constants::{DEFAULT_TOAST_TIMEOUT, INFINITE_TOAST_TIMEOUT};
use crate::model::node::{DecryptedNode, NodeMetadata};
use crate::utils::ui::shorten_file_name;
use crabdrive_common::storage::NodeType;
use leptos::prelude::*;
use thaw::{
    Button, Menu, MenuItem, MenuTrigger, MenuTriggerType, Toast, ToastIntent, ToastOptions,
    ToastTitle, ToasterInjection,
};
use web_sys::File;

#[component]
pub(crate) fn ModifyNodeMenu(
    #[prop(into)] node: Signal<DecryptedNode>,
    #[prop(into)] parent: Signal<DecryptedNode>,
    on_modified: Callback<()>,
) -> impl IntoView {
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
    let folder_selection_dialog_open = RwSignal::new(false);
    let input_dialog_open = RwSignal::new(false);
    let metadata = Signal::derive(move || {
        let NodeMetadata::V1(metadata) = node.get().metadata;
        metadata
    });

    let rename_action = Action::new_local(move |input: &String| {
        let new_name = input.to_owned();
        async move {
            rename_node(node.get_untracked(), new_name)
                .await
                .map_err(|err| err.to_string())
        }
    });
    Effect::new(move || {
        let status = rename_action.value().get();
        if status.is_some() {
            match status.unwrap() {
                Ok(_) => on_modified.run(()),
                Err(e) => add_toast(format!("Failed to rename: {}", e), ToastIntent::Error),
            }
        }
    });

    let move_action = Action::new_local(move |input: &DecryptedNode| {
        let target = input.to_owned();
        async move {
            move_node(node.get_untracked(), parent.get_untracked(), target)
                .await
                .map_err(|err| err.to_string())
        }
    });
    Effect::new(move || {
        let status = move_action.value().get();
        if status.is_some() {
            match status.unwrap() {
                Ok(_) => on_modified.run(()),
                Err(e) => add_toast(format!("Failed to move: {}", e), ToastIntent::Error),
            }
        }
    });

    let move_to_trash_action = Action::new_local(move |_| async move {
        move_node_to_trash(node.get_untracked())
            .await
            .map_err(|err| err.to_string())
    });
    Effect::new(move || {
        let status = move_to_trash_action.value().get();
        if status.is_some() {
            match status.unwrap() {
                Ok(_) => {
                    add_toast(
                        "Moved to trash successfully".to_string(),
                        ToastIntent::Success,
                    );
                    on_modified.run(())
                }
                Err(e) => add_toast(
                    format!("Failed to move to trash: {}", e),
                    ToastIntent::Error,
                ),
            }
        }
    });

    let on_select = move |key: &str| match key {
        "rename" => input_dialog_open.set(true),
        "move" => folder_selection_dialog_open.set(true),
        "new_revision" => file_selection_dialog_open.set(true),
        "move_to_trash" => (move || {
            move_to_trash_action.dispatch(());
        })(),
        _ => add_toast("TODO".to_owned(), ToastIntent::Error),
    };

    view! {
        <Menu on_select trigger_type=MenuTriggerType::Hover>
            <MenuTrigger slot>
                <Button icon=icondata_mdi::MdiPencilOutline block=true>
                    "Modify"
                </Button>
            </MenuTrigger>
            <MenuItem value="rename" icon=icondata_mdi::MdiRenameOutline>
                "Rename"
            </MenuItem>
            <MenuItem value="move" icon=icondata_mdi::MdiArrowAll>
                "Move"
            </MenuItem>
            <Show when=move || node.get().node_type == NodeType::File>
                <MenuItem value="new_revision" icon=icondata_mdi::MdiFileReplaceOutline>
                    "Upload new version"
                </MenuItem>
            </Show>
            <MenuItem value="move_to_trash" icon=icondata_mdi::MdiDeleteOutline>
                "Move to trash"
            </MenuItem>
        </Menu>
        <InputDialog
            open=input_dialog_open
            title=Signal::derive(move || format!("Rename '{}'", metadata.get().name))
            confirm_label="Rename"
            on_confirm=Callback::new(move |new_name| {
                rename_action.dispatch(new_name);
                input_dialog_open.set(false);
            })
        />
        <FileSelectionDialog
            open=file_selection_dialog_open
            on_confirm=Callback::new(move |_files: Vec<File>| {
                add_toast("TODO".to_owned(), ToastIntent::Error);
                file_selection_dialog_open.set(false)
            })
            title=Signal::derive(move || {
                format!("Upload new revision of '{}'", shorten_file_name(metadata.get().name))
            })
            allow_multiple=false
        />
        <FolderSelectionDialog
            open=folder_selection_dialog_open
            on_confirm=Callback::new(move |selected_node: DecryptedNode| {
                move_action.dispatch(selected_node);
                folder_selection_dialog_open.set(false)
            })
            title=Signal::derive(move || {
                format!("Select destination for '{}'", shorten_file_name(metadata.get().name))
            })
            confirm_label="Move here"
            start_folder=parent
        />
    }
}
