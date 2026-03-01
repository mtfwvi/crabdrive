use crate::components::data_provider::accepted_nodes_provider::AcceptedNodesProvider;
use crate::components::data_provider::path_provider::PathProvider;
use crate::components::data_provider::trash_provider::TrashProvider;
use crate::components::folder_view::FolderView;
use crate::components::shared_view::SharedView;
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
        <Show when=move || content_type.get() == ContentViewType::Trash>
            <TrashProvider let:trash_node let:refetch>
                <TrashView trash_node request_trash_node_refetch=refetch />
            </TrashProvider>
        </Show>
        <Show when=move || content_type.get() == ContentViewType::Shared>
            <AcceptedNodesProvider let:accepted_nodes let:refetch>
                <SharedView accepted_nodes request_accepted_nodes_refetch=refetch />
            </AcceptedNodesProvider>
        </Show>
    }
}
