use crate::api::download_file;
use crate::components::file_selection_dialog::FileSelectionDialog;
use crate::model::node::DecryptedNode;
use crate::model::node::NodeMetadata;
use crate::utils::ui::format_date_time;
use leptos::prelude::*;
use thaw::{
    Button, ButtonAppearance, Divider, Space, Text, Toast, ToastIntent, ToastOptions, ToastTitle,
    ToasterInjection,
};
use web_sys::FileList;

#[component]
pub(crate) fn FileDetails(
    #[prop(into)] selection: Signal<DecryptedNode>,
    on_close: Callback<()>,
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
        let NodeMetadata::V1(metadata) = selection.get().metadata;
        metadata
    });

    let download_action = Action::new_local(|input: &DecryptedNode| {
        let node = input.to_owned();
        async move { download_file(node).await }
    });
    let handle_download = move |_| {
        download_action.dispatch(selection.get().clone());
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
        <Space vertical=true>
            <Space class="my-3 content-center justify-between">
                <Text class="!text-2xl !font-bold">{move || metadata.get().name}</Text>
                <Button
                    appearance=ButtonAppearance::Subtle
                    class="!min-w-0 ml-2"
                    on_click=move |_| on_close.run(())
                    icon=icondata::MdiClose
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

            <Divider class="my-3" />
            <Space class="flex-1">
                <Button
                    on_click=handle_download
                    appearance=ButtonAppearance::Primary
                    icon=icondata::MdiDownload
                >
                    "Download"
                </Button>
                <Button
                    on_click=move |_| file_selection_dialog_open.set(true)
                    icon=icondata::MdiFileReplaceOutline
                >
                    "Modify"
                </Button>
            </Space>
        </Space>
        <FileSelectionDialog
            open=file_selection_dialog_open
            on_confirm=Callback::new(move |file_list: FileList| {
                add_toast(format!("Received file_list with file to be uploaded: {:?}", file_list));
                file_selection_dialog_open.set(false)
            })
            title=Signal::derive(move || {
                format!("Upload new revision of {}", &metadata.get().name)
            })
            allow_multiple=false
        />
    }
}
