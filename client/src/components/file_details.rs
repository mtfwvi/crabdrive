use crate::components::file_selection_dialog::FileSelectionDialog;
use crate::model::node::DecryptedNode;
use crate::model::node::NodeMetadata;
use crate::utils::ui::format_date_time;
use icondata::AiCloseOutlined;
use leptos::prelude::*;
use thaw::{
    Button, ButtonAppearance, Divider, Space, Text, Toast, ToastIntent, ToastOptions, ToastTitle,
    ToasterInjection,
};

#[component]
pub(crate) fn FileDetails(
    #[prop(into)] selection: RwSignal<Option<DecryptedNode>>,
) -> impl IntoView {
    let file_selection_dialog_open = RwSignal::new(false);

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
            ToastOptions::default().with_intent(ToastIntent::Info),
        )
    };

    let metadata = Signal::derive(move || {
        // `selection.get()` cannot be None, because FileDetails is wrapped by Show
        let NodeMetadata::V1(metadata) = selection.get().unwrap().metadata;
        metadata
    });

    view! {
        <Space vertical=true>
            <Space class="my-3 content-center justify-between">
                <Text class="!text-2xl !font-bold">{move || metadata.get().name}</Text>
                <Button
                    appearance=ButtonAppearance::Subtle
                    class="!min-w-0 ml-2"
                    on_click=move |_| selection.set(None)
                    icon=AiCloseOutlined
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

            // <Divider class="my-3" />
            // <Space class="content-center">
            // <Avatar name="dercodeling" size=25 />
            // <Text class="!text-lg !font-medium">"dercodeling (owner)"</Text>
            // </Space>

            <Divider class="my-3" />
            <Space class="flex-1">
                <Button
                    on_click=move |_| add_toast(String::from("TODO"))
                    appearance=ButtonAppearance::Primary
                    icon=icondata::AiCloudDownloadOutlined
                >
                    "Download"
                </Button>
                <Button
                    on_click=move |_| file_selection_dialog_open.set(true)
                    icon=icondata::AiDiffOutlined
                >
                    "Modify"
                </Button>
            </Space>

            <FileSelectionDialog
                open=file_selection_dialog_open
                on_confirm=move |file_list| {
                    add_toast(format!("Received file_list to be uploaded: {:?}", file_list));
                    file_selection_dialog_open.set(false)
                }
                title=Signal::derive(move || {
                    format!("Upload new revision of {}", &metadata.get().name)
                })
                allow_multiple=false
            />
        </Space>
    }
}
