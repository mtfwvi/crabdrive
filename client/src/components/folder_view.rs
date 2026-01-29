use crate::api::get_children;
use crate::components::file_details::FileDetails;
use crate::components::file_list::FileList;
use crate::components::file_selection_dialog::FileSelectionDialog;
use crate::components::folder_creation_dialog::FolderCreationDialog;
use crate::components::path_breadcrumb::PathBreadcrumb;
use crate::components::resource_wrapper::ResourceWrapper;
use crate::constants::EMPTY_KEY;
use crate::model::node::{DecryptedNode, MetadataV1, NodeMetadata};
use chrono::Utc;
use crabdrive_common::storage::{NodeId, NodeType};
use crabdrive_common::uuid::UUID;
use leptos::prelude::*;
use thaw::{
    Button, ButtonAppearance, Divider, Layout, LayoutSider, Space, Toast, ToastBody, ToastIntent,
    ToastOptions, ToasterInjection,
};

#[component]
pub(crate) fn FolderView(#[prop(into)] node_id: Signal<NodeId>) -> impl IntoView {
    let toaster = ToasterInjection::expect_context();

    let files_res = LocalResource::new(move || get_children(node_id.get()));

    // TODO: Load real data
    let node_res = LocalResource::new(move || async {
        let root_node_metadata = NodeMetadata::V1(MetadataV1 {
            name: "root".to_string(),
            last_modified: Utc::now().naive_utc(),
            created: Default::default(),
            size: None,
            mime_type: None,
            file_key: None,
            children_key: vec![],
        });

        let decrypted_node = DecryptedNode {
            id: UUID::random(),
            change_count: 0,
            parent_id: UUID::random(),
            owner_id: UUID::nil(),
            deleted_on: None,
            node_type: NodeType::Folder,
            current_revision: None,
            metadata: root_node_metadata,
            encryption_key: EMPTY_KEY,
        };
        Ok(decrypted_node)
    });

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

    view! {
        <Layout class="h-fit flex-1 rounded-sm outline outline-gray-300" has_sider=true>
            <Space vertical=true class="flex-1 flex-column gap-3 p-8">
                <ResourceWrapper
                    resource=node_res
                    error_text=String::from("The node info could not be loaded from the server")
                    render=move |node| view! { <PathBreadcrumb node /> }.into_any()
                    fallback_spinner=false
                />

                <Divider class="mb-3" />

                <ResourceWrapper
                    resource=files_res
                    error_text=Signal::derive(move || format!("The children of {} could not be loaded from the server", node_id.get()))
                    render=move |files| view! { <FileList files selection /> }.into_any()
                />

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
                        let NodeMetadata::V1(metadata) = node_res.get().unwrap().unwrap().metadata;
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
        </Layout>
    }
}
