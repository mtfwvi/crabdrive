use crate::components::data_provider::children_provider::ChildrenProvider;
use crate::components::node_details::{DetailsViewType, NodeDetails};
use crate::components::node_list::NodeList;
use crate::components::trash_empty_button::TrashEmptyButton;
use crate::model::node::DecryptedNode;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use thaw::{Divider, Icon, Space, SpaceAlign, Text};

#[component]
pub fn TrashView(
    trash_node: Signal<DecryptedNode>,
    request_trash_node_refetch: Callback<()>,
) -> impl IntoView {
    let navigate = use_navigate();
    let navigate_to_node = Callback::new(move |node: DecryptedNode| {
        navigate(&format!("/{}", node.id), Default::default());
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
        <ChildrenProvider node=trash_node let:children let:refetch_children>
            <Space vertical=true class="flex-1 flex-column p-8 gap-3 justify-between">
                <Space vertical=true>
                    <Space align=SpaceAlign::Center>
                        <Icon class="!text-2xl mr-1" icon=icondata_mdi::MdiTrashCanOutline />
                        <Text class="!text-2xl !font-bold">"Trash"</Text>
                    </Space>
                    <Divider class="mb-3" />

                    <NodeList
                        nodes=children
                        no_nodes_message="Trash is empty"
                        on_node_click=toggle_selection
                        on_folder_dblclick=navigate_to_node
                        folders_only=false
                    />
                </Space>

                // Request refetch since parent metadata was modified
                <TrashEmptyButton on_emptied=request_trash_node_refetch />
            </Space>

            <Show when=move || selection.get().is_some()>
                <NodeDetails
                    node=Signal::derive(move || selection.get().unwrap())
                    content_type=DetailsViewType::Trash
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
