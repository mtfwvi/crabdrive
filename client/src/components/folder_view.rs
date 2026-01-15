use crate::components::file_details::FileDetails;
use crate::components::file_list::FileList;
use crate::components::path_breadcrumb::PathBreadcrumb;
use leptos::prelude::*;
use thaw::{
    Button, ButtonAppearance, Divider, Layout, LayoutSider, Space, Toast, ToastIntent,
    ToastOptions, ToastTitle, ToasterInjection,
};

#[component]
pub(crate) fn FolderView() -> impl IntoView {
    let toaster = ToasterInjection::expect_context();

    let add_toast = move |_| {
        toaster.dispatch_toast(
            move || {
                view! {
                    <Toast>
                        <ToastTitle>"TODO"</ToastTitle>
                    </Toast>
                }
            },
            ToastOptions::default().with_intent(ToastIntent::Error),
        )
    };

    let (files, _set_files) = signal(
        [
            "README.md",
            "audio.mp3",
            "document.pdf",
            "garbage.txt",
            "FlameShot-v0.1.0-x86_64.deb",
        ]
        .iter()
        .map(|str| str.to_string())
        .collect(),
    );
    let (path, _set_path) = signal(
        ["home", "jonathan", "Downloads"]
            .iter()
            .map(|str| str.to_string())
            .collect(),
    );

    let selection = RwSignal::new(String::new());

    view! {
        <Layout class="flex-1 rounded-sm outline outline-gray-300" has_sider=true>
            <Space vertical=true class="flex-1 flex-column gap-3 p-8">
                <PathBreadcrumb node_names=path />
                <Divider class="mb-3" />
                <FileList files selection />
                <Divider class="my-3" />
                <Space>
                    <Button
                        on_click=add_toast
                        appearance=ButtonAppearance::Primary
                        icon=icondata::AiPlusOutlined
                    >
                        Upload file
                    </Button>
                    <Button on_click=add_toast icon=icondata::AiFolderAddOutlined>
                        Create folder
                    </Button>
                </Space>
            </Space>

            <Show when=move || !selection.get().is_empty()>
                <LayoutSider class="border-l-1 border-gray-200 p-5">
                    <FileDetails selection />
                </LayoutSider>
            </Show>
        </Layout>
    }
}
