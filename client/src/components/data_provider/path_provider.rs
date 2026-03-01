use crate::api::get_accessible_path;
use crate::components::basic::resource_wrapper::ResourceWrapper;
use crate::model::node::DecryptedNode;
use crabdrive_common::storage::NodeId;
use leptos::prelude::*;

#[component]
pub fn PathProvider<C, V>(
    #[prop(into)] node_id: Signal<NodeId>,
    children: C,
) -> impl IntoView
where
    C: Fn(Signal<Vec<DecryptedNode>>, Callback<()>) -> V + Send + Sync + 'static,
    V: IntoView + 'static,
{
    let path_res = LocalResource::new(move || async move {
        get_accessible_path(node_id.get())
            .await
            .map_err(|err| err.to_string())
    });

    let refetch = Callback::new(move |_| path_res.refetch());

    view! {
        <ResourceWrapper
            resource=path_res
            error_text=Signal::derive(move || {
                format!("The path to node '{}' could not be loaded from the server", node_id.get())
            })
            children=move |path| children(path, refetch)
        />
    }
}
