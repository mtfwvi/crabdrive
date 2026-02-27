use crate::api::get_children;
use crate::components::basic::resource_wrapper::ResourceWrapper;
use crate::model::node::DecryptedNode;
use leptos::prelude::*;

#[component]
pub(crate) fn ChildrenProvider<C, V>(node: Signal<DecryptedNode>, children: C) -> impl IntoView
where
    C: Fn(Signal<Vec<DecryptedNode>>, Callback<()>) -> V + Send + Sync + 'static,
    V: IntoView + 'static,
{
    let children_res = LocalResource::new(move || async move {
        get_children(node.get())
            .await
            .map_err(|err| err.to_string())
    });

    let refetch = Callback::new(move |_| children_res.refetch());

    view! {
        <ResourceWrapper
            resource=children_res
            error_text=Signal::derive(move || {
                format!("Failed to load children of {} from server", node.get().id)
            })
            fallback_spinner=false
            children=move |children_nodes| children(children_nodes, refetch)
        />
    }
}
