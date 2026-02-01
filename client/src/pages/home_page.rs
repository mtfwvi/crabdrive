use crate::components::folder_view::FolderView;
use crabdrive_common::storage::NodeId;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use thaw::{Image, Layout, LayoutSider, Space, SpaceAlign, Text};

#[component]
pub(crate) fn HomePage() -> impl IntoView {
    let node_id: Signal<Option<NodeId>> =
        Signal::derive(move || use_params_map().get().get("id").map(|str| str.into()));

    view! {
        <Layout content_style="padding: 30px 40px; height: 100vh" has_sider=true>
            <LayoutSider class="!min-w-73">
                <Space align=SpaceAlign::Center>
                    <Image src="/logo.svg" attr:width=50 />
                    <Text class="!text-3xl !font-bold">"crabdrive"</Text>
                </Space>
                <Text class="!text-lg !font-bold">"Rust native cloud storage"</Text>
            </LayoutSider>

            <Layout class="h-fit min-h-57 flex-1 p-8 rounded-sm outline outline-gray-300" has_sider=true>
                <Show
                    when=move || node_id.get().is_some()
                    fallback=|| view! { <Text>No node selected.</Text> }
                >
                    <FolderView node_id=Signal::derive(move || node_id.get().unwrap()) />
                </Show>
            </Layout>
        </Layout>
    }
}
