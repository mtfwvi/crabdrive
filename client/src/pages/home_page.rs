use crate::components::folder_view::FolderView;
use leptos::prelude::*;
use thaw::{ConfigProvider, Image, Layout, LayoutSider, Space, SpaceAlign};

#[component]
pub(crate) fn HomePage() -> impl IntoView {
    view! {
        <ConfigProvider>
            <Layout content_style="padding: 20px;" has_sider=true>
                <LayoutSider>
                    <Space align=SpaceAlign::Center>
                        <Image src="/logo.svg" attr:width=50 />
                        <h1>"crabdrive"</h1>
                    </Space>
                    <h3>"Rust native cloud storage"</h3>
                </LayoutSider>

                <FolderView />
            </Layout>
        </ConfigProvider>
    }
}
