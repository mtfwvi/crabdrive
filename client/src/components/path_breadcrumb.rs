use crate::model::node::{DecryptedNode, NodeMetadata};
use crabdrive_common::storage::NodeId;
use leptos::prelude::*;
use thaw::{Breadcrumb, BreadcrumbButton, BreadcrumbDivider, BreadcrumbItem, Text};

#[component]
pub(crate) fn PathBreadcrumb(
    #[prop(into)] path: Signal<Vec<DecryptedNode>>,
    on_select: Callback<NodeId>,
    #[prop(optional, default = false)] compact: bool,
) -> impl IntoView {
    let current_node = move || path.get().last().expect("Path was empty").clone();

    let inner_node_style = if compact { "!text-lg" } else { "!text-xl" };
    let leaf_node_style = if compact { "!text-xl" } else { "!text-2xl" };

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
                            leaf_node_style
                            inner_node_style
                        />
                        <Show when=is_not_last>
                            <BreadcrumbDivider class=inner_node_style />
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
    leaf_node_style: &'static str,
    inner_node_style: &'static str,
    on_click: Callback<NodeId>,
) -> impl IntoView {
    let on_click = move |_| on_click.run(node.get().id);

    let name = Signal::derive(move || {
        let NodeMetadata::V1(metadata) = node.get().metadata;
        metadata.name
    });

    let text_style = Signal::derive(move || {
        if is_last.get() {
            leaf_node_style
        } else {
            inner_node_style
        }
    });

    view! {
        <BreadcrumbItem>
            <BreadcrumbButton on:click=on_click>
                <Text class=format!("{} !font-bold", text_style.get())>{name}</Text>
            </BreadcrumbButton>
        </BreadcrumbItem>
    }
}
