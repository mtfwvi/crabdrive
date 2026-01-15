use crate::display_utils::format_date_time;
use chrono::Utc;
use icondata::AiCloseOutlined;
use leptos::prelude::*;
use thaw::{
    Button, ButtonAppearance, Divider, Space, Text, Toast, ToastIntent, ToastOptions, ToastTitle,
    ToasterInjection,
};

#[component]
pub(crate) fn FileDetails(
    #[prop(into)] file: Signal<String>, // TODO: Switch String out for proper FileDetails once type exists
    set_selected_file: WriteSignal<String>,
) -> impl IntoView {
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

    view! {
        <Space vertical=true>
            <Space class="my-3 content-center justify-between">
                <Text class="!text-2xl !font-bold">{file}</Text>
                <Button
                    appearance=ButtonAppearance::Subtle
                    class="!min-w-0 ml-2"
                    on_click=move |_| set_selected_file.set(String::new())
                    icon=AiCloseOutlined
                />
            </Space>

            <Text>"Size: 86 KB"</Text>
            <Text>"Last modified: "{format_date_time(Utc::now().naive_utc())}</Text>
            <Text>Created: {format_date_time(Utc::now().naive_utc())}</Text>

            // <Divider class="my-3" />
            // <Space class="content-center">
            // <Avatar name="dercodeling" size=25 />
            // <Text class="!text-lg !font-medium">"dercodeling (owner)"</Text>
            // </Space>

            <Divider class="my-3" />
            <Space class="flex-1">
                <Button
                    on_click=add_toast
                    appearance=ButtonAppearance::Primary
                    icon=icondata::AiCloudDownloadOutlined
                >
                    Download
                </Button>
                <Button on_click=add_toast icon=icondata::AiDiffOutlined>
                    Upload new version
                </Button>
            </Space>
        </Space>
    }
}
