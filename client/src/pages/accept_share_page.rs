use crate::api::accept_share;
use crate::utils::browser::{get_current_url, set_current_url};
use leptos::{IntoView, component};

#[component]
pub fn AcceptSharePage() -> impl IntoView {
    leptos::reactive::spawn_local(async move {
        let url = get_current_url().unwrap();
        let node_id = accept_share(&url).await.unwrap();

        set_current_url(&format!("/{}", node_id)).unwrap();
    });
}
