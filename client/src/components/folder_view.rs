use crate::api::{get_children, path_to_root};
use crate::components::file_details::FileDetails;
use crate::components::file_selection_dialog::FileSelectionDialog;
use crate::components::folder_creation_dialog::FolderCreationDialog;
use crate::components::node_list::NodeList;
use crate::components::path_breadcrumb::PathBreadcrumb;
use crate::components::resource_wrapper::ResourceWrapper;
use crate::model::node::{DecryptedNode, NodeMetadata};
use crabdrive_common::storage::{NodeId, NodeType};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use thaw::{
    Button, ButtonAppearance, Divider, LayoutSider, Space, Toast, ToastBody, ToastIntent,
    ToastOptions, ToasterInjection,
};

#[component]
pub(crate) fn FolderView(#[prop(into)] node_id: Signal<NodeId>) -> impl IntoView {
    let toaster = ToasterInjection::expect_context();
    let navigate = use_navigate();

    let children_res = LocalResource::new(move || get_children(node_id.get()));
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
    let selection: RwSignal<Option<DecryptedNode>> = RwSignal::new(None);

    let current_node_from =
        move |path: Vec<DecryptedNode>| path.last().expect("Failed due to empty path").clone();

    let navigate_to_node = Callback::new(move |node_id| {
        navigate(&format!("/{}", node_id), Default::default());
        selection.set(None);
    });

    let toggle_selection = move |file: DecryptedNode| {
        let selected = selection.get().clone();
        let is_selected = selected.is_some() && selected.unwrap().id == file.id;

        selection.set(if is_selected {
            None
        } else {
            Some(file.clone())
        });
    };

    let on_select_node = Callback::new(move |node: DecryptedNode| match node.node_type {
        NodeType::File => toggle_selection(node),
        NodeType::Folder => navigate_to_node.run(node.id),
        NodeType::Link => add_toast(String::from("Links have not been implemented")),
    });

    view! {
        <ResourceWrapper
            resource=path_res
            error_text=Signal::derive(move || {
                format!("The node '{}' could not be loaded from the server", node_id.get())
            })
            fallback_spinner=false
            let:path
        >
            <Space vertical=true class="flex-1 flex-column gap-3 justify-between">
                <Space vertical=true>
                    <PathBreadcrumb path on_select=navigate_to_node />
                    <Divider class="mb-3" />

                    <ResourceWrapper
                        resource=children_res
                        error_text=Signal::derive(move || {
                            format!(
                                "The children of '{}' could not be loaded from the server",
                                node_id.get(),
                            )
                        })
                        let:children
                    >
                        <NodeList nodes=children on_select=on_select_node />
                    </ResourceWrapper>
                </Space>

                <Space vertical=true>
                    <Divider class="my-3" />

                    <Space>
                        <Button
                            on_click=move |_| file_selection_dialog_open.set(true)
                            appearance=ButtonAppearance::Primary
                            icon=icondata::MdiPlus
                        >
                            "Upload file"
                        </Button>
                        <Button
                            on_click=move |_| folder_creation_dialog_open.set(true)
                            icon=icondata::MdiFolderPlusOutline
                        >
                            "Create folder"
                        </Button>
                    </Space>
                </Space>
            </Space>

            <Show when=move || selection.get().is_some()>
                <LayoutSider>
                    <Space class="!gap-0">
                        <Divider class="mx-5" vertical=true/>
                        <FileDetails selection />
                    </Space>
                </LayoutSider>
            </Show>

            <FileSelectionDialog
                open=file_selection_dialog_open
                on_confirm=move |file_list| {
                    add_toast(format!("Received {} file(s) to upload", file_list.length()));
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
