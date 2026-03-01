use crate::components::file_creation_button::FileCreationButton;
use crate::components::folder_creation_button::FolderCreationButton;
use crate::model::node::DecryptedNode;
use leptos::prelude::*;
use thaw::{Divider, Space};

#[component]
pub fn FolderBottomBar(
    #[prop(into)] current_node: Signal<DecryptedNode>,
    on_children_modified: Callback<()>,
) -> impl IntoView {
    view! {
        <Space vertical=true>
            <Divider class="my-3" />
            <Space>
                <FileCreationButton
                    parent_node=Signal::derive(move || current_node.get())
                    on_created=on_children_modified
                />
                <FolderCreationButton
                    parent_node=Signal::derive(move || current_node.get())
                    on_created=on_children_modified
                />
            </Space>
        </Space>
    }
}
