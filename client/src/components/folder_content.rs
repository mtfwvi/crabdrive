use crate::api::get_children;
use crate::components::basic::resource_wrapper::ResourceWrapper;
use crate::components::folder_bottom_bar::FolderBottomBar;
use crate::components::node_list::NodeList;
use crate::components::path_breadcrumb::PathBreadcrumb;
use crate::model::node::DecryptedNode;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use thaw::{Divider, Space};

#[component]
pub(crate) fn FolderContent(
    path: Signal<Vec<DecryptedNode>>,
    on_select_node: Callback<DecryptedNode>,
    on_content_modified: Callback<()>,
) -> impl IntoView {
    let navigate = use_navigate();
    let navigate_to_node = Callback::new(move |node_id| {
        navigate(&format!("/{}", node_id), Default::default());
    });

    let current_node = Signal::derive(move || {
        path.get()
            .last()
            .expect("Failed to get current node due to empty path")
            .clone()
    });
    let children_res = LocalResource::new(move || {
        let current_node = current_node.get();
        async move {
            get_children(current_node)
                .await
                .map_err(|err| err.to_string())
        }
    });

    view! {
        <Space vertical=true class="flex-1 flex-column p-8 gap-3 justify-between">
            <Space vertical=true>
                <PathBreadcrumb path is_trash=false on_select=navigate_to_node />
                <Divider class="mb-3" />

                <ResourceWrapper
                    resource=children_res
                    error_text=Signal::derive(move || {
                        format!(
                            "The children of '{}' could not be loaded from the server",
                            current_node.get().id,
                        )
                    })
                    let:children
                >
                    <NodeList
                        nodes=children
                        on_node_click=on_select_node
                        on_folder_dblclick=navigate_to_node
                        folders_only=false
                    />
                </ResourceWrapper>
            </Space>

            <FolderBottomBar current_node is_trash=false on_children_changed=on_content_modified />
        </Space>
    }
}
