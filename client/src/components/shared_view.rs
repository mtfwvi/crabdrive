use crate::components::node_details::{DetailsViewType, NodeDetails};
use crate::components::node_list::NodeList;
use crate::model::node::DecryptedNode;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use thaw::{Divider, Icon, Space, SpaceAlign, Text};

#[component]
pub fn SharedView(
    accepted_nodes: Signal<Vec<DecryptedNode>>,
    request_accepted_nodes_refetch: Callback<()>,
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
        <Space vertical=true class="flex-1 flex-column p-8 gap-3 justify-start">
            <Space align=SpaceAlign::Center>
                <Icon class="!text-2xl mr-1" icon=icondata_mdi::MdiFolderAccountOutline />
                <Text class="!text-2xl !font-bold">"Shared with you"</Text>
            </Space>
            <Divider class="mb-3" />

            <NodeList
                nodes=accepted_nodes
                no_nodes_message="No shares accepted so far"
                on_node_click=toggle_selection
                on_folder_dblclick=navigate_to_node
                folders_only=false
            />
        </Space>

        <Show when=move || selection.get().is_some()>
            <NodeDetails
                node=Signal::derive(move || selection.get().unwrap())
                content_type=DetailsViewType::Shared
                on_close=Callback::new(move |_| selection.set(None))
                on_modified=Callback::new(move |_| {
                    request_accepted_nodes_refetch.run(());
                    selection.set(None);
                })
            />
        </Show>
    }
}
