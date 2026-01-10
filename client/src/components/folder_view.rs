use crate::components::file_details::FileDetails;
use crate::components::file_list::FileList;
use crate::components::path_breadcrumb::PathBreadcrumb;
use leptos::prelude::*;
use thaw::{Divider, Layout, LayoutSider, Space};

#[component]
pub(crate) fn FolderView() -> impl IntoView {
    let (files, _set_files) = signal(
        vec!["README.md", "audio.mp3", "document.pdf"]
            .iter()
            .map(|str| str.to_string())
            .collect(),
    );
    let (path, _set_path) = signal(
        vec!["home", "jonathan", "Downloads"]
            .iter()
            .map(|str| str.to_string())
            .collect(),
    );

    let (selected_file, set_selected_file) = signal(String::new());

    view! {
        <Layout
            attr:style="flex: 1; box-shadow: none; outline: 1px solid lightgray; border-radius: 5px"
            has_sider=true
        >
            <Space
                vertical=true
                attr:style="flex-direction: column; gap: 10px; padding: 30px; flex: 1"
            >
                <PathBreadcrumb node_names=path />
                <Divider />
                <FileList files=files set_selected_file />
            </Space>

            <Show when=move || !selected_file.get().is_empty()>
                <LayoutSider attr:style="border-left: 1px dotted lightgray; padding: 20px">
                    <FileDetails file=selected_file set_selected_file />
                </LayoutSider>
            </Show>
        </Layout>
    }
}
