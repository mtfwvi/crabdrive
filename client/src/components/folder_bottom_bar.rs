use crate::components::file_creation_button::FileCreationButton;
use crate::components::folder_creation_button::FolderCreationButton;
use crate::model::node::DecryptedNode;
use leptos::prelude::*;
use thaw::{Divider, Space};

#[component]
pub(crate) fn FolderBottomBar(
    #[prop(into)] current_node: Signal<DecryptedNode>,
    is_trash: Signal<bool>, // TODO: Use
    on_children_changed: Callback<()>,
) -> impl IntoView {
    view! {
        <Space vertical=true>
            <Divider class="my-3" />

            <Space>
                <FileCreationButton
                    parent_node=Signal::derive(move || current_node.get())
                    on_created=on_children_changed
                />
                <FolderCreationButton
                    parent_node=Signal::derive(move || current_node.get())
                    on_created=on_children_changed
                />
            </Space>
        </Space>
    }
}
