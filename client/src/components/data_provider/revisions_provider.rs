use crate::api::file_versions;
use crate::components::basic::resource_wrapper::ResourceWrapper;
use crate::model::node::DecryptedNode;
use crabdrive_common::storage::FileRevision;
use leptos::prelude::*;

#[component]
pub fn RevisionsProvider<C, V>(node: Signal<DecryptedNode>, children: C) -> impl IntoView
where
    C: Fn(Signal<Vec<FileRevision>>) -> V + Send + Sync + 'static,
    V: IntoView + 'static,
{
    let revisions_res = LocalResource::new(move || async move {
        file_versions(node.get_untracked().id)
            .await
            .map_err(|err| err.to_string())
    });

    view! {
        <ResourceWrapper
            resource=revisions_res
            error_text="Failed to load shared nodes from server"
            children=move |revisions| children(revisions)
        />
    }
}
