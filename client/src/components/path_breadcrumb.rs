use crate::model::node::{DecryptedNode, NodeMetadata};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use thaw::{Breadcrumb, BreadcrumbButton, BreadcrumbDivider, BreadcrumbItem, Text};

#[component]
pub(crate) fn PathBreadcrumb(#[prop(into)] node: Signal<DecryptedNode>) -> impl IntoView {
    // TODO: Talk about making this path of node (i.e. to its root), since the server knows the root easily but I don't want to know it really
    let path = Signal::derive(move || vec![node.get()]); // TODO: Get real path

    view! {
        <Breadcrumb>
            <For
                each=move || path.get()
                key=|path_node| path_node.id
                children=move |path_node| {
                    let is_not_last = move || path_node.id != node.get().id;

                    view! {
                        <PathBreadcrumbItem node=path_node is_last=!is_not_last() />
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
) -> impl IntoView {
    let navigate = use_navigate();

    let on_click = move |_| navigate(&format!("/{}", node.get().id), Default::default());

    let name = Signal::derive(move || {
        let NodeMetadata::V1(metadata) = node.get().metadata;
        metadata.name
    });

    view! {
        <BreadcrumbItem>
            <BreadcrumbButton on:click=on_click>
                <Text class=format!(
                    "!{} !font-bold",
                    if is_last.get() { "text-3xl" } else { "text-2xl" },
                )>{name}</Text>
            </BreadcrumbButton>
        </BreadcrumbItem>
    }
}
