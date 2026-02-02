use crate::api::{get_children, path_to_root};
use crate::components::file_creation_button::FileCreationButton;
use crate::components::file_details::FileDetails;
use crate::components::folder_creation_button::FolderCreationButton;
use crate::components::node_list::NodeList;
use crate::components::path_breadcrumb::PathBreadcrumb;
use crate::components::resource_wrapper::ResourceWrapper;
use crate::model::node::DecryptedNode;
use crabdrive_common::storage::{NodeId, NodeType};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use std::time::Duration;
use thaw::{
    Divider, LayoutSider, Space, Toast, ToastBody, ToastIntent, ToastOptions, ToasterInjection,
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
            ToastOptions::default()
                .with_intent(ToastIntent::Info)
                .with_timeout(Duration::from_millis(10_000)),
        )
    };

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
                        <FileCreationButton
                            parent_node=Signal::derive(move || current_node_from(path.get()))
                            on_created=Callback::new(move |_| {
                                children_res.refetch();
                                path_res.refetch()
                            })
                        />
                        <FolderCreationButton
                            parent_node=Signal::derive(move || current_node_from(path.get()))
                            on_created=Callback::new(move |_| {
                                children_res.refetch();
                                path_res.refetch()
                            })
                        />
                    </Space>
                </Space>
            </Space>

            <Show when=move || selection.get().is_some()>
                <LayoutSider>
                    <Space class="!gap-0">
                        <Divider class="mx-5" vertical=true />
                        <FileDetails
                            selection=Signal::derive(move || selection.get().unwrap())
                            on_close=Callback::new(move |_| selection.set(None))
                        />
                    </Space>
                </LayoutSider>
            </Show>
        </ResourceWrapper>
    }
}
