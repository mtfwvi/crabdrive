use crate::components::folder_view::FolderView;
use crate::components::path_provider::PathProvider;
use crate::components::trash_provider::TrashProvider;
use crate::components::trash_view::TrashView;
use crabdrive_common::storage::NodeId;
use leptos::prelude::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) enum ContentViewType {
    Folder(NodeId),
    Shared,
    Trash,
}

#[component]
pub(crate) fn ContentFrame(#[prop(into)] content_type: Signal<ContentViewType>) -> impl IntoView {
    view! {
        <Show when=move || matches!(content_type.get(), ContentViewType::Folder(_))>
            <PathProvider
                node_id=Signal::derive(move || {
                    let ContentViewType::Folder(node_id) = content_type.get() else {
                        unreachable!()
                    };
                    node_id
                })
                let:path
                let:refetch
            >
                <FolderView path request_path_refetch=refetch />
            </PathProvider>
        </Show>
        // TODO: Add SharedView
        <Show when=move || content_type.get() == ContentViewType::Trash>
            <TrashProvider let:trash_node let:refetch>
                <TrashView trash_node request_trash_node_refetch=refetch />
            </TrashProvider>
        </Show>
    }
}
