use crate::components::file_selection_dialog::FileSelectionDialog;
use crate::constants::DEFAULT_TOAST_TIMEOUT;
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
pub(crate) fn ModifyNodeMenu(#[prop(into)] node: Signal<DecryptedNode>) -> impl IntoView {
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

    let file_selection_dialog_open = RwSignal::new(false);
    let metadata = Signal::derive(move || {
        let NodeMetadata::V1(metadata) = node.get().metadata;
        metadata
    });

    let on_select = move |key: &str| match key {
        "new_revision" => file_selection_dialog_open.set(true),
        "rename" => add_toast("TODO".to_owned()),
        "move" => add_toast("TODO".to_owned()),
        "move_to_trash" => add_toast("TODO".to_owned()),
        _ => add_toast("TODO".to_owned()),
    };

    view! {
        <Menu on_select trigger_type=MenuTriggerType::Hover>
            <MenuTrigger slot>
                <Button
                    on_click=move |_| file_selection_dialog_open.set(true)
                    icon=icondata_mdi::MdiPencilOutline
                    block=true
                >
                    "Modify"
                </Button>
            </MenuTrigger>
            <Show when=move || node.get().node_type == NodeType::File>
                <MenuItem value="new_revision" icon=icondata_mdi::MdiFileReplaceOutline>
                    "Upload new version"
                </MenuItem>
            </Show>
            <MenuItem value="rename" icon=icondata_mdi::MdiRenameOutline>
                "Rename"
            </MenuItem>
            <MenuItem value="move" icon=icondata_mdi::MdiArrowAll>
                "Move"
            </MenuItem>
            <MenuItem value="move_to_trash" icon=icondata_mdi::MdiDeleteOutline>
                "Move to trash"
            </MenuItem>
        </Menu>
        <FileSelectionDialog
            open=file_selection_dialog_open
            on_confirm=Callback::new(move |_files: Vec<File>| {
                add_toast("TODO".to_owned());
                file_selection_dialog_open.set(false)
            })
            title=Signal::derive(move || {
                format!("Upload new revision of {}", shorten_file_name(metadata.get().name))
            })
            allow_multiple=false
        />
    }
}
