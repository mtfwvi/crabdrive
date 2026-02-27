use crate::components::data_provider::children_provider::ChildrenProvider;
use crate::components::folder_bottom_bar::FolderBottomBar;
use crate::components::node_details::{DetailsViewType, NodeDetails};
use crate::components::node_list::NodeList;
use crate::components::path_breadcrumb::PathBreadcrumb;
use crate::model::node::DecryptedNode;
use crate::utils::browser::SessionStorage;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use thaw::{Divider, Space};

#[component]
pub(crate) fn FolderView(
    path: Signal<Vec<DecryptedNode>>,
    request_path_refetch: Callback<()>,
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
    let path_root_node = Signal::derive(move || {
        path.get()
            .first()
            .expect("Failed to get path root due to empty path")
            .clone()
    });

    let selection: RwSignal<Option<DecryptedNode>> = RwSignal::new(None);

    let _reset_selection_effect = Effect::watch(
        move || path.get(),
        move |_, _, _| selection.set(None),
        false,
    );

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
        <ChildrenProvider node=current_node let:children let:refetch_children>
            <Space vertical=true class="flex-1 flex-column p-8 gap-3 justify-between">
                <Space vertical=true>
                    <PathBreadcrumb path on_select=navigate_to_node />
                    <Divider class="mb-3" />

                    <NodeList
                        nodes=children
                        on_node_click=toggle_selection
                        on_folder_dblclick=navigate_to_node
                        folders_only=false
                    />
                </Space>

                // Request refetch since parent metadata was modified
                <FolderBottomBar current_node on_children_modified=request_path_refetch />
            </Space>

            <Show when=move || selection.get().is_some()>
                <NodeDetails
                    node=Signal::derive(move || selection.get().unwrap())
                    content_type=Signal::derive(move || {
                        let trash_id = SessionStorage::get("trash_id").unwrap_or_default();
                        if trash_id == Some(path_root_node.get().id) {
                            DetailsViewType::ReadOnly
                        } else {
                            DetailsViewType::Folder
                        }
                    })
                    on_close=Callback::new(move |_| selection.set(None))
                    on_modified=Callback::new(move |_| {
                        refetch_children.run(());
                        selection.set(None);
                    })
                />
            </Show>
        </ChildrenProvider>
    }
}
