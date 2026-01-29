use crate::api::{get_children, path_to_root};
use crate::components::file_details::FileDetails;
use crate::components::file_list::FileList;
use crate::components::file_selection_dialog::FileSelectionDialog;
use crate::components::folder_creation_dialog::FolderCreationDialog;
use crate::components::path_breadcrumb::PathBreadcrumb;
use crate::components::resource_wrapper::ResourceWrapper;
use crate::model::node::{DecryptedNode, NodeMetadata};
use crabdrive_common::storage::NodeId;
use leptos::prelude::*;
use thaw::{
    Button, ButtonAppearance, Divider, LayoutSider, Space, Toast, ToastBody, ToastIntent,
    ToastOptions, ToasterInjection,
};

#[component]
pub(crate) fn FolderView(#[prop(into)] node_id: Signal<NodeId>) -> impl IntoView {
    let toaster = ToasterInjection::expect_context();

    let files_res = LocalResource::new(move || get_children(node_id.get()));
    let path_res = LocalResource::new(move || path_to_root(node_id.get()));

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
    let folder_creation_dialog_open = RwSignal::new(false);
    let selection = RwSignal::new(None);

    let current_node_from =
        move |path: Vec<DecryptedNode>| path.last().expect("Failed due to empty path").clone();

    view! {
        <ResourceWrapper
            resource=path_res
            error_text=Signal::derive(move || format!("The node '{}' could not be loaded from the server", node_id.get()))
            fallback_spinner=false
            let:path
        >
            <Space vertical=true class="flex-1 flex-column gap-3">
                <PathBreadcrumb path />

                <Divider class="mb-3" />

                <ResourceWrapper
                    resource=files_res
                    error_text=Signal::derive(move || format!("The children of '{}' could not be loaded from the server", node_id.get()))
                    let:files
                >
                    <FileList files selection />
                </ResourceWrapper>

                <Divider class="my-3" />

                <Space>
                    <Button
                        on_click=move |_| file_selection_dialog_open.set(true)
                        appearance=ButtonAppearance::Primary
                        icon=icondata::AiPlusOutlined
                    >
                        "Upload file"
                    </Button>
                    <Button
                        on_click=move |_| folder_creation_dialog_open.set(true)
                        icon=icondata::AiFolderAddOutlined
                    >
                        "Create folder"
                    </Button>
                </Space>
            </Space>

            <Show when=move || selection.get().is_some()>
                <LayoutSider class="border-l-1 border-gray-200 p-5">
                    <FileDetails selection />
                </LayoutSider>
            </Show>

            <FileSelectionDialog
                open=file_selection_dialog_open
                on_confirm=move |file_list| {
                    add_toast(format!("Received file_list to be uploaded: {:?}", file_list));
                    file_selection_dialog_open.set(false)
                }
                title=Signal::derive(move || {
                    let name = Signal::derive(move || {
                        let NodeMetadata::V1(metadata) = current_node_from(path.get()).metadata;
                        metadata.name
                    });

                    format!("Upload files to {}", name.get())
                })
            />

            <FolderCreationDialog
                open=folder_creation_dialog_open
                on_confirm=move |name| {
                    add_toast(format!("Received folder name '{}'", name));
                    folder_creation_dialog_open.set(false)
                }
            />
        </ResourceWrapper>
    }
}
