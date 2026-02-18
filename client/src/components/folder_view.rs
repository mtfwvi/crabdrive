use crate::api::{get_children, get_trash_node, path_to_root};
use crate::components::folder_bottom_bar::FolderBottomBar;
use crate::components::node_details::NodeDetails;
use crate::components::node_list::NodeList;
use crate::components::path_breadcrumb::PathBreadcrumb;
use crate::components::resource_wrapper::ResourceWrapper;
use crate::model::node::DecryptedNode;
use crabdrive_common::storage::NodeId;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use thaw::{Divider, Space};

#[component]
pub(crate) fn FolderView(
    #[prop(into)] node_id: Signal<NodeId>,
    is_trash: Signal<bool>,
) -> impl IntoView {
    let navigate = use_navigate();
    let selection: RwSignal<Option<DecryptedNode>> = RwSignal::new(None);

    let _reset_selection_effect = Effect::watch(
        move || node_id.get(),
        move |_, _, _| selection.set(None),
        false,
    );

    let path_res = LocalResource::new(move || {
        let node_id = node_id.get();
        async move {
            if is_trash.get() {
                let trash_node = get_trash_node().await.map_err(|err| err.to_string())?;
                Ok(vec![trash_node])
            } else {
                path_to_root(node_id).await.map_err(|err| err.to_string())
            }
        }
    });

    let navigate_to_node = Callback::new(move |node_id| {
        navigate(&format!("/{}", node_id), Default::default());
    });

    let toggle_selection = Callback::new(move |file: DecryptedNode| {
        let selected = selection.get().clone();
        let is_selected = selected.is_some() && selected.unwrap().id == file.id;

        selection.set(if is_selected {
            None
        } else {
            Some(file.clone())
        });
    });

    let on_children_changed = Callback::new(move |_| {
        path_res.refetch()
        // children_res will automatically refetch, because it is
        // only created within the resource wrapper for path_res
    });

    view! {
        <ResourceWrapper
            resource=path_res
            error_text=Signal::derive(move || {
                format!("The path to node '{}' could not be loaded from the server", node_id.get())
            })
            fallback_spinner=false
            children=move |path| {
                let current_node = Signal::derive(move || {
                    path.get().last().expect("Failed to get current node due to empty path").clone()
                });
                let children_res = LocalResource::new(move || {
                    let current_node = current_node.get();
                    async move { get_children(current_node).await.map_err(|err| err.to_string()) }
                });

                view! {
                    <Space vertical=true class="flex-1 flex-column p-8 gap-3 justify-between">
                        <Space vertical=true>
                            <PathBreadcrumb path is_trash on_select=navigate_to_node />
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
                                <NodeList
                                    nodes=children
                                    on_node_click=toggle_selection
                                    on_folder_dblclick=navigate_to_node
                                    folders_only=false
                                />
                            </ResourceWrapper>
                        </Space>

                        <FolderBottomBar current_node is_trash on_children_changed />
                    </Space>

                    <Show when=move || selection.get().is_some()>
                        <NodeDetails
                            node=Signal::derive(move || selection.get().unwrap())
                            parent=current_node
                            is_trash
                            on_close=Callback::new(move |_| selection.set(None))
                            on_modified=Callback::new(move |_| {
                                children_res.refetch();
                                selection.set(None);
                            })
                        />
                    </Show>
                }
            }
        />
    }
}
