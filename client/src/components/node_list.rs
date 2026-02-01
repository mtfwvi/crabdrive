use crate::model::node::{DecryptedNode, NodeMetadata};
use leptos::prelude::*;
use thaw::{Button, ButtonAppearance, Text};

#[component]
pub(crate) fn NodeList(
    #[prop(into)] nodes: Signal<Vec<DecryptedNode>>,
    on_select: Callback<DecryptedNode>,
) -> impl IntoView {
    let on_click = move |node: DecryptedNode| on_select.run(node);

    view! {
        <For
            each=move || nodes.get()
            key=|node| node.id
            children=move |node| {
                let (node, _) = signal(node);
                view! {
                    <NodeListItem
                        name=Signal::derive(move || {
                            let NodeMetadata::V1(metadata) = node.get().metadata;
                            metadata.name
                        })
                        on:click=move |_| on_click(node.get())
                    />
                }
            }
        />
    }
}

#[component]
fn NodeListItem(#[prop(into)] name: Signal<String>) -> impl IntoView {
    view! {
        <Button appearance=ButtonAppearance::Subtle class="w-full flex !justify-start">
            <Text>{name}</Text>
        </Button>
    }
}
