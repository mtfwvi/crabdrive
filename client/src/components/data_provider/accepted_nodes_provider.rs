use crate::api::get_accepted_nodes;
use crate::components::basic::resource_wrapper::ResourceWrapper;
use crate::model::node::DecryptedNode;
use leptos::prelude::*;

#[component]
pub(crate) fn AcceptedNodesProvider<C, V>(children: C) -> impl IntoView
where
    C: Fn(Signal<Vec<DecryptedNode>>, Callback<()>) -> V + Send + Sync + 'static,
    V: IntoView + 'static,
{
    let accepted_nodes_res = LocalResource::new(move || async move {
        get_accepted_nodes().await.map_err(|err| err.to_string())
    });

    let refetch = Callback::new(move |_| accepted_nodes_res.refetch());

    view! {
        <ResourceWrapper
            resource=accepted_nodes_res
            error_text="Failed to load shared nodes from server"
            children=move |accepted_nodes| children(accepted_nodes, refetch)
        />
    }
}
