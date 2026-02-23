use crate::api::{get_trash_node, path_to_root};
use crate::components::basic::resource_wrapper::ResourceWrapper;
use crate::components::content_frame::ContentViewType;
use crate::model::node::DecryptedNode;
use leptos::prelude::*;

#[component]
pub(crate) fn PathProvider<C, V>(
    #[prop(into)] content_type: Signal<ContentViewType>,
    children: C,
) -> impl IntoView
where
    C: Fn(Signal<Vec<DecryptedNode>>, Callback<()>) -> V + Send + Sync + 'static,
    V: IntoView + 'static,
{
    let path_res = LocalResource::new(move || async move {
        match content_type.get() {
            ContentViewType::Folder(node_id) => {
                path_to_root(node_id).await.map_err(|err| err.to_string())
            }
            ContentViewType::Trash => get_trash_node()
                .await
                .map_err(|err| err.to_string())
                .map(|trash_node| vec![trash_node]),
            ContentViewType::Shared => Ok(vec![]),
        }
    });

    let refetch = Callback::new(move |_| path_res.refetch());

    view! {
        <ResourceWrapper
            resource=path_res
            error_text=Signal::derive(move || {
                match content_type.get() {
                    ContentViewType::Folder(node_id) => {
                        format!(
                            "The path to node '{}' could not be loaded from the server",
                            node_id,
                        )
                    }
                    ContentViewType::Shared => {
                        String::from("Failed to get path because view_type is Shared, not Folder")
                    }
                    ContentViewType::Trash => {
                        String::from("Failed to get path because view_type is Trash, not Folder")
                    }
                }
            })
            fallback_spinner=false
            children=move |path| children(path, refetch)
        />
    }
}
