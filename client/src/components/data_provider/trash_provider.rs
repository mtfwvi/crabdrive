use crate::api::get_trash_node;
use crate::components::basic::resource_wrapper::ResourceWrapper;
use crate::model::node::DecryptedNode;
use leptos::prelude::*;

#[component]
pub(crate) fn TrashProvider<C, V>(children: C) -> impl IntoView
where
    C: Fn(Signal<DecryptedNode>, Callback<()>) -> V + Send + Sync + 'static,
    V: IntoView + 'static,
{
    let trash_res =
        LocalResource::new(
            move || async move { get_trash_node().await.map_err(|err| err.to_string()) },
        );

    let refetch = Callback::new(move |_| trash_res.refetch());

    view! {
        <ResourceWrapper
            resource=trash_res
            error_text="Failed to load trash items from server"
            children=move |trash_node| children(trash_node, refetch)
        />
    }
}
