use crate::api::download_file;
use crate::components::modify_node_menu::ModifyNodeMenu;
use crate::constants::DEFAULT_TOAST_TIMEOUT;
use crate::model::node::DecryptedNode;
use crate::model::node::NodeMetadata;
use crate::utils::ui::{format_date_time, shorten_file_name};
use crabdrive_common::storage::NodeType;
use leptos::prelude::*;
use thaw::{
    Button, ButtonAppearance, Space, Text, Toast, ToastIntent, ToastOptions, ToastTitle,
    ToasterInjection,
};

#[component]
pub(crate) fn FileDetails(
    #[prop(into)] node: Signal<DecryptedNode>,
    on_modified: Callback<()>,
    on_close: Callback<()>,
) -> impl IntoView {
    let toaster = ToasterInjection::expect_context();

    let add_toast = move |text: String| {
        toaster.dispatch_toast(
            move || {
                view! {
                    <Toast>
                        <ToastTitle>{text}</ToastTitle>
                    </Toast>
                }
            },
            ToastOptions::default()
                .with_intent(ToastIntent::Error)
                .with_timeout(DEFAULT_TOAST_TIMEOUT),
        )
    };

    let metadata = Signal::derive(move || {
        let NodeMetadata::V1(metadata) = node.get().metadata;
        metadata
    });

    let download_action = Action::new_local(|input: &DecryptedNode| {
        let node = input.to_owned();
        async move { download_file(node).await.map_err(|err| err.to_string()) }
    });
    let handle_download = move |_| {
        download_action.dispatch(node.get().clone());
    };

    Effect::new(move || {
        let status = download_action.value().get();
        if status.is_some() {
            let response = status.unwrap();
            if response.is_err() {
                add_toast(format!(
                    "Failed to download {}: {}",
                    metadata.get().name,
                    response.err().unwrap()
                ))
            }
        }
    });

    view! {
        <Space vertical=true class="p-8 !max-w-[30vw]">
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

            <Show when=move || metadata.get().mime_type.is_some()>
                <Text>{move || format!("Type: {}", metadata.get().mime_type.unwrap())}</Text>
            </Show>
            <Show when=move || metadata.get().size.is_some()>
                <Text>{move || format!("Size: {}", metadata.get().size.unwrap())}</Text>
            </Show>
            <Text>
                {move || {
                    format!("Last modified: {}", format_date_time(metadata.get().last_modified))
                }}
            </Text>
            <Text>{move || format!("Created: {}", format_date_time(metadata.get().created))}</Text>

            <Space vertical=true class="mt-4">
                <Show when=move || node.get().node_type == NodeType::File>
                    <Button
                        on_click=handle_download
                        appearance=ButtonAppearance::Primary
                        icon=icondata_mdi::MdiDownload
                        block=true
                    >
                        "Download"
                    </Button>
                </Show>
                <ModifyNodeMenu node on_modified />
            </Space>
        </Space>
    }
}
