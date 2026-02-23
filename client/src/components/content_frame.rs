use crate::components::folder_content::FolderContent;
use crate::components::path_provider::PathProvider;
use crabdrive_common::storage::NodeId;
use leptos::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub(crate) enum ContentViewType {
    Folder(NodeId),
    Shared,
    Trash,
}

#[component]
pub(crate) fn ContentFrame(#[prop(into)] content_type: Signal<ContentViewType>) -> impl IntoView {
    match content_type.get() {
        ContentViewType::Folder(_) => view! {
            <PathProvider content_type let:path let:refetch>
                <FolderContent path request_path_refetch=refetch />
            </PathProvider>
        }
        .into_any(),
        ContentViewType::Shared => {
            view! { <p>todo</p> }.into_any() // TODO
        }
        ContentViewType::Trash => view! { <p>todo</p> }.into_any(),
    }
}
