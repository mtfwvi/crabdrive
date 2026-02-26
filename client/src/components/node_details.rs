use crate::components::content_frame::ContentViewType;
use crate::components::file_download_button::FileDownloadButton;
use crate::components::modify_node_menu::ModifyNodeMenu;
use crate::model::node::DecryptedNode;
use crate::model::node::NodeMetadata;
use crate::utils::ui::{
    format_date_time, get_owner_username, get_share_acceptor_usernames, shorten_file_name,
};
use crabdrive_common::storage::NodeType;
use leptos::prelude::*;
use thaw::{Button, ButtonAppearance, Divider, LayoutSider, Space, Text};

#[component]
pub(crate) fn NodeDetails(
    #[prop(into)] node: Signal<DecryptedNode>,
    #[prop(into)] content_type: Signal<ContentViewType>,
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
                        name="Owner"
                        value=Signal::derive(move || get_owner_username(node.get()))
                    />
                    <OptionalNodeAttribute
                        name="Access"
                        value=Signal::derive(move || {
                            get_share_acceptor_usernames(node.get())
                                .map(|usernames| usernames.join(", "))
                        })
                    />

                    <Show when=move || matches!(content_type.get(), ContentViewType::Folder(_))>
                        <Space vertical=true class="mt-4">
                            <Show when=move || node.get().node_type == NodeType::File>
                                <FileDownloadButton node />
                            </Show>

                            <ModifyNodeMenu node on_modified />
                        </Space>
                    </Show>

                    <Show when=move || {
                        content_type.get() == ContentViewType::Shared
                            && node.get().node_type == NodeType::File
                    }>
                        <FileDownloadButton node />
                    </Show>

                    <Show when=move || content_type.get() == ContentViewType::Trash>
                        <Space vertical=true class="mt-4">
                            <Button
                                block=true
                                icon=icondata_mdi::MdiRestore
                                appearance=ButtonAppearance::Primary
                            >
                                "Restore"
                            </Button>
                            <Button block=true icon=icondata_mdi::MdiDeleteForeverOutline>
                                "Delete forever"
                            </Button>
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
