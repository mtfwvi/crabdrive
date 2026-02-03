use crate::model::node::{DecryptedNode, NodeMetadata};
use crabdrive_common::storage::NodeId;
use leptos::prelude::*;
use thaw::{Breadcrumb, BreadcrumbButton, BreadcrumbDivider, BreadcrumbItem, Text};

#[component]
pub(crate) fn PathBreadcrumb(
    #[prop(into)] path: Signal<Vec<DecryptedNode>>,
    on_select: Callback<NodeId>,
) -> impl IntoView {
    let current_node = move || path.get().last().expect("Path was empty").clone();

    view! {
        <Breadcrumb>
            <For
                each=move || path.get()
                key=|path_node| path_node.id
                children=move |path_node| {
                    let is_not_last = move || path_node.id != current_node().id;

                    view! {
                        <PathBreadcrumbItem
                            node=path_node
                            is_last=!is_not_last()
                            on_click=on_select
                        />
                        <Show when=is_not_last>
                            <BreadcrumbDivider class="!text-xl" />
                        </Show>
                    }
                }
            />
        </Breadcrumb>
    }
}

#[component]
fn PathBreadcrumbItem(
    #[prop(into)] node: Signal<DecryptedNode>,
    #[prop(optional, into)] is_last: Signal<bool>,
    on_click: Callback<NodeId>,
) -> impl IntoView {
    let on_click = move |_| on_click.run(node.get().id);

    let name = Signal::derive(move || {
        let NodeMetadata::V1(metadata) = node.get().metadata;
        metadata.name
    });

    view! {
        <BreadcrumbItem>
            <BreadcrumbButton on:click=on_click>
                <Text class=format!(
                    "!{} !font-bold",
                    if is_last.get() { "text-2xl" } else { "text-xl" },
                )>{name}</Text>
            </BreadcrumbButton>
        </BreadcrumbItem>
    }
}
