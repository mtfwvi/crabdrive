use crate::components::file_creation_button::FileCreationButton;
use crate::components::folder_creation_button::FolderCreationButton;
use crate::components::trash_empty_button::TrashEmptyButton;
use crate::model::node::DecryptedNode;
use leptos::prelude::*;
use thaw::{Divider, Space};

#[component]
pub(crate) fn FolderBottomBar(
    #[prop(into)] current_node: Signal<DecryptedNode>,
    is_trash: Signal<bool>,
    on_children_changed: Callback<()>,
) -> impl IntoView {
    view! {
        <Show when=move || !is_trash.get() fallback=move || view! { <TrashEmptyButton /> }>
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
        </Show>
    }
}
