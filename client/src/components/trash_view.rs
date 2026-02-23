use crate::api::get_children;
use crate::components::basic::resource_wrapper::ResourceWrapper;
use crate::components::folder_bottom_bar::FolderBottomBar;
use crate::components::node_details::NodeDetails;
use crate::components::node_list::NodeList;
use crate::components::path_breadcrumb::PathBreadcrumb;
use crate::model::node::DecryptedNode;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use thaw::{Divider, Space};

#[component]
pub(crate) fn TrashView(
    trash_node: Signal<DecryptedNode>,
    request_trash_node_refetch: Callback<()>,
) -> impl IntoView {
    let navigate = use_navigate();
    let navigate_to_node = Callback::new(move |node_id| {
        navigate(&format!("/{}", node_id), Default::default());
    });

    let children_res = LocalResource::new(move || {
        let current_node = trash_node.get();
        async move {
            get_children(current_node)
                .await
                .map_err(|err| err.to_string())
        }
    });

    let selection: RwSignal<Option<DecryptedNode>> = RwSignal::new(None);
    let toggle_selection = Callback::new(move |file: DecryptedNode| {
        let selected = selection.get().clone();
        let is_selected = selected.is_some() && selected.unwrap().id == file.id;

        selection.set(if is_selected {
            None
        } else {
            Some(file.clone())
        });
    });

    view! {
        <ResourceWrapper
            resource=children_res
            error_text="The items in trash could not be loaded from the server"
            let:children
        >
            <Space vertical=true class="flex-1 flex-column p-8 gap-3 justify-between">
                <Space vertical=true>
                    // TODO: Refactor breadcrumbs to title
                    <PathBreadcrumb
                        path=Signal::derive(move || vec![trash_node.get()])
                        is_trash=true
                        on_select=navigate_to_node
                    />
                    <Divider class="mb-3" />

                    <NodeList
                        nodes=children
                        on_node_click=toggle_selection
                        on_folder_dblclick=navigate_to_node
                        folders_only=false
                    />
                </Space>

                // Request refetch since parent metadata was modified
                // TODO: Refactor: separate bar based on basic component, pass children from here
                <FolderBottomBar
                    current_node=trash_node
                    is_trash=true
                    on_children_changed=request_trash_node_refetch
                />
            </Space>

            <Show when=move || selection.get().is_some()>
                <NodeDetails
                    node=Signal::derive(move || selection.get().unwrap())
                    parent=trash_node
                    // TODO: Implement passing view_type
                    is_trash=true
                    on_close=Callback::new(move |_| selection.set(None))
                    on_modified=Callback::new(move |_| {
                        children_res.refetch();
                        selection.set(None);
                    })
                />
            </Show>
        </ResourceWrapper>
    }
}
