use crate::components::folder_view::FolderView;
use leptos::prelude::*;
use thaw::{Image, Layout, LayoutSider, Space, SpaceAlign, Text};

#[component]
pub(crate) fn HomePage() -> impl IntoView {
    view! {
        <Layout content_style="padding: 30px 40px;" has_sider=true>
            <LayoutSider>
                <Space align=SpaceAlign::Center>
                    <Image src="/logo.svg" attr:width=50 />
                    <Text class="!text-3xl !font-bold">"crabdrive"</Text>
                </Space>
                <Text class="!text-lg !font-bold">"Rust native cloud storage"</Text>
            </LayoutSider>

            <FolderView />
        </Layout>
    }
}
