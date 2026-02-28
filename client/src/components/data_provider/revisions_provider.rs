use crate::components::basic::resource_wrapper::ResourceWrapper;
use crate::model::node::DecryptedNode;
use chrono::Local;
use crabdrive_common::storage::{FileRevision, RevisionIv};
use crabdrive_common::uuid::UUID;
use leptos::prelude::*;

#[component]
pub(crate) fn RevisionsProvider<C, V>(node: Signal<DecryptedNode>, children: C) -> impl IntoView
where
    C: Fn(Signal<Vec<FileRevision>>) -> V + Send + Sync + 'static,
    V: IntoView + 'static,
{
    let revisions_res = LocalResource::new(move || async move {
        Ok(std::iter::repeat_n(
            node.get_untracked()
                .current_revision
                .unwrap_or(FileRevision {
                    // TODO: Use real data
                    id: UUID::nil(),
                    upload_ended_on: Some(Local::now().naive_local()),
                    upload_started_on: Local::now().naive_local(),
                    iv: RevisionIv::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                    chunk_count: 1,
                }),
            3,
        )
        .collect())
    });

    view! {
        <ResourceWrapper
            resource=revisions_res
            error_text="Failed to load shared nodes from server"
            fallback_spinner=false
            children=move |revisions| children(revisions)
        />
    }
}
