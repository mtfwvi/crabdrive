use crate::constants::DEFAULT_TOAST_TIMEOUT;
use crate::model::node::{DecryptedNode, NodeMetadata};
use crate::utils::ui::get_node_icon;
use crabdrive_common::storage::{NodeId, NodeType};
use leptos::prelude::*;
use thaw::{
    Button, ButtonAppearance, ButtonSize, Flex, FlexGap, FlexJustify, Text, Toast, ToastIntent,
    ToastOptions, ToastTitle, ToasterInjection,
};

#[component]
pub(crate) fn NodeList(
    #[prop(into)] nodes: Signal<Vec<DecryptedNode>>,
    on_select: Callback<DecryptedNode>,
    on_open_folder: Callback<NodeId>,
) -> impl IntoView {
    let toaster = ToasterInjection::expect_context();

    let add_toast = move |text: String| {
        toaster.dispatch_toast(
            move || {
                view! {
                    <Toast>
                        <ToastTitle>{text}</ToastTitle>
                    </Toast>
                }
            },
            ToastOptions::default()
                .with_intent(ToastIntent::Info)
                .with_timeout(DEFAULT_TOAST_TIMEOUT),
        )
    };
    let on_dblclick = move |node: DecryptedNode| match node.node_type {
        NodeType::File => {}
        NodeType::Folder => on_open_folder.run(node.id),
        NodeType::Link => add_toast(String::from("Links have not been implemented")),
    };

    let sorted_nodes = move |node_type: NodeType| {
        let all_nodes = nodes.get();
        let mut filtered_nodes: Vec<DecryptedNode> = all_nodes
            .into_iter()
            .filter(|node| node.node_type == node_type)
            .collect();

        filtered_nodes.sort_by_key(|node| {
            let NodeMetadata::V1(metadata) = node.metadata.clone();
            metadata.name
        });

        filtered_nodes
    };

    view! {
        <Show
            when=move || !nodes.get().is_empty()
            fallback=|| view! { <Text>"Folder is empty"</Text> }
        >
            <Flex vertical=true gap=FlexGap::Large justify=FlexJustify::FlexStart>
                <For
                    each=move || vec![NodeType::Folder, NodeType::File, NodeType::Link]
                    key=|node_type| *node_type
                    let:node_type
                >
                    // For grouping each node type's nodes together
                    <div class=if sorted_nodes(node_type).is_empty() { "hidden" } else { "" }>
                        <For
                            each=move || sorted_nodes(node_type)
                            key=|node| node.id
                            children=move |node| {
                                let node = Signal::derive(move || node.clone());
                                view! {
                                    <NodeListItem
                                        name=Signal::derive(move || {
                                            let NodeMetadata::V1(metadata) = node.get().metadata;
                                            metadata.name
                                        })
                                        node_type=Signal::derive(move || node.get().node_type)
                                        on:click=move |_| on_select.run(node.get())
                                        on:dblclick=move |e| {
                                            e.prevent_default();
                                            on_dblclick(node.get())
                                        }
                                    />
                                }
                            }
                        />
                    </div>
                </For>
            </Flex>
        </Show>
    }
}

#[component]
fn NodeListItem(
    #[prop(into)] name: Signal<String>,
    #[prop(into)] node_type: Signal<NodeType>,
) -> impl IntoView {
    view! {
        <Button
            appearance=ButtonAppearance::Subtle
            icon=Signal::derive(move || get_node_icon(node_type.get(), name.get()))
            size=Signal::derive(move || {
                if node_type.get() == NodeType::Folder {
                    ButtonSize::Large
                } else {
                    ButtonSize::Medium
                }
            })
            class="w-full flex !justify-start !px-4 !py-1"
        >
            <Text>{name}</Text>
        </Button>
    }
}
