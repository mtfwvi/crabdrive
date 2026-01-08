use crate::components::file_tree::FileTree;
use leptos::prelude::*;
use thaw::{Badge, ConfigProvider, Layout, LayoutHeader, Space};

#[component]
pub(crate) fn DemoPage() -> impl IntoView {
    view! {
        <ConfigProvider>
            <Layout content_style="max-width: 800px; padding: 20px;">
                <LayoutHeader>
                    <h1>"crabdrive 🦀"</h1>
                    <Space>
                        <h3 style="display: inline; padding-right: 5px;">
                            "Rust native cloud storage"
                        </h3>
                        <Badge>"⚙️"</Badge>
                    </Space>
                </LayoutHeader>
                <Layout content_style="padding: 20px;">
                    <FileTree />
                </Layout>
            </Layout>
        </ConfigProvider>
    }
}
