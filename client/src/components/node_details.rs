use crate::components::file_download_button::FileDownloadButton;
use crate::components::file_history_button::FileHistoryButton;
use crate::components::modify_node_menu::ModifyNodeMenu;
use crate::components::node_share_button::NodeShareButton;
use crate::components::trash_item_delete_button::TrashItemDeleteButton;
use crate::components::trash_item_restore_button::TrashItemRestoreButton;
use crate::model::node::DecryptedNode;
use crate::model::node::NodeMetadata;
use crate::utils::ui::{
    format_date_time, get_owner_username, get_share_acceptor_usernames, shorten_file_name,
};
use crabdrive_common::storage::NodeType;
use leptos::prelude::*;
use thaw::{Button, ButtonAppearance, Divider, LayoutSider, Space, Text};

#[derive(PartialEq, Clone, Debug)]
pub(crate) enum DetailsViewType {
    Folder(Box<DecryptedNode>), // parent node
    Shared,
    Trash,
    ReadOnly,
}

#[component]
pub(crate) fn NodeDetails(
    #[prop(into)] node: Signal<DecryptedNode>,
    #[prop(into)] content_type: Signal<DetailsViewType>,
    on_modified: Callback<()>,
    on_close: Callback<()>,
) -> impl IntoView {
    let metadata = Signal::derive(move || {
        let NodeMetadata::V1(metadata) = node.get().metadata;
        metadata
    });

    view! {
        <LayoutSider content_style="height: 100%">
            <Space class="!gap-0 h-full">
                <Divider vertical=true />

                <Space vertical=true class="p-8 !min-w-[25vw] !max-w-[35vw]">
                    <Space class="my-3 content-center justify-between">
                        <Text class="!text-2xl !font-bold">
                            {move || shorten_file_name(metadata.get().name)}
                        </Text>
                        <Button
                            appearance=ButtonAppearance::Subtle
                            class="!min-w-0 ml-2"
                            on_click=move |_| on_close.run(())
                            icon=icondata_mdi::MdiClose
                        />
                    </Space>

                    <OptionalNodeAttribute
                        name="Type"
                        value=Signal::derive(move || {
                            metadata.get().mime_type.map(|size| size.to_string())
                        })
                    />
                    <OptionalNodeAttribute
                        name="Size"
                        value=Signal::derive(move || {
                            metadata.get().size.map(|size| size.to_string())
                        })
                    />
                    <NodeAttribute
                        name="Last modified"
                        value=Signal::derive(move || format_date_time(metadata.get().last_modified))
                    />
                    <NodeAttribute
                        name="Created"
                        value=Signal::derive(move || format_date_time(metadata.get().created))
                    />
                    <OptionalNodeAttribute
                        name="Deleted"
                        value=Signal::derive(move || node.get().deleted_on.map(format_date_time))
                    />
                    <OptionalNodeAttribute
                        name="Owner"
                        value=Signal::derive(move || get_owner_username(node.get()))
                    />
                    <OptionalNodeAttribute
                        name="Shared with"
                        value=Signal::derive(move || {
                            get_share_acceptor_usernames(node.get())
                                .map(|usernames| usernames.join(", "))
                        })
                    />

                    <Show when=move || matches!(content_type.get(), DetailsViewType::Folder(_))>
                        <Space vertical=true class="mt-4">
                            <Show when=move || node.get().node_type == NodeType::File>
                                <FileDownloadButton node />
                            </Show>

                            <ModifyNodeMenu
                                node
                                parent=Signal::derive(move || {
                                    match content_type.get() {
                                        DetailsViewType::Folder(parent) => *parent,
                                        _ => unreachable!(),
                                    }
                                })
                                on_modified
                            />

                            <Show when=move || node.get().node_type == NodeType::File>
                                <FileHistoryButton node />
                            </Show>

                            <NodeShareButton node />
                        </Space>
                    </Show>

                    <Show when=move || {
                        content_type.get() == DetailsViewType::Shared
                            && node.get().node_type == NodeType::File
                    }>
                        <Space vertical=true class="mt-4">
                            <FileDownloadButton node />
                            <FileHistoryButton node />
                        </Space>
                    </Show>

                    <Show when=move || content_type.get() == DetailsViewType::Trash>
                        <Space vertical=true class="mt-4">
                            <TrashItemRestoreButton node on_restored=on_modified />
                            <TrashItemDeleteButton node on_deleted=on_modified />
                        </Space>
                    </Show>
                </Space>
            </Space>
        </LayoutSider>
    }
}

#[component]
fn OptionalNodeAttribute(
    #[prop(into)] name: Signal<String>,
    #[prop(into)] value: Signal<Option<String>>,
) -> impl IntoView {
    view! {
        <Show when=move || value.get().is_some()>
            <NodeAttribute name value=Signal::derive(move || value.get().unwrap()) />
        </Show>
    }
}

#[component]
fn NodeAttribute(
    #[prop(into)] name: Signal<String>,
    #[prop(into)] value: Signal<String>,
) -> impl IntoView {
    view! { <Text>{move || format!("{}: {}", name.get(), value.get())}</Text> }
}
