use crate::components::folder_view::FolderView;
use crabdrive_common::storage::NodeId;
use crabdrive_common::uuid::UUID;
use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params_map};
use thaw::{Image, Layout, LayoutSider, Space, SpaceAlign, Text};

#[component]
pub(crate) fn HomePage() -> impl IntoView {
    let navigate = use_navigate();
    let root_node_id = UUID::nil().to_string(); // TODO: Get root for user

    let node_id: Signal<NodeId> = Signal::derive(move || {
        let node_parameter = use_params_map().get().get("id");
        node_parameter
            .unwrap_or_else(|| {
                navigate(&format!("/{}", root_node_id), Default::default());
                root_node_id.clone()
            })
            .into()
    });

    view! {
        <Layout content_style="padding: 30px 40px; height: 100vh" has_sider=true>
            <LayoutSider class="!min-w-73">
                <Space align=SpaceAlign::Center>
                    <Image src="/logo.svg" attr:width=50 />
                    <Text class="!text-3xl !font-bold">"crabdrive"</Text>
                </Space>
                <Text class="!text-lg !font-bold">"Rust native cloud storage"</Text>
            </LayoutSider>

            <FolderView node_id />
        </Layout>
    }
}
