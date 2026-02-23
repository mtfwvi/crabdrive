use crate::components::folder_view::FolderView;
use crate::components::path_provider::PathProvider;
use crate::components::trash_provider::TrashProvider;
use crate::components::trash_view::TrashView;
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
        ContentViewType::Folder(node_id) => view! {
            <PathProvider node_id let:path let:refetch>
                <FolderView path request_path_refetch=refetch />
            </PathProvider>
        }
        .into_any(),
        ContentViewType::Shared => {
            view! { <p>todo</p> }.into_any() // TODO
        }
        ContentViewType::Trash => view! {
            <TrashProvider let:trash_node let:refetch>
                <TrashView trash_node request_trash_node_refetch=refetch />
            </TrashProvider>
        }
        .into_any(),
    }
}
