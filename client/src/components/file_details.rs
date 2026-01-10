use crate::display_utils::format_date_time;
use chrono::Utc;
use leptos::prelude::*;
use thaw::{Button, ButtonAppearance, Space, Text};

#[component]
pub(crate) fn FileDetails(
    #[prop(into)] file: Signal<String>, // TODO: Switch String out for proper FileDetails once type exists
    set_selected_file: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <Space vertical=true>
            <Space attr:style="align-content: center; justify-content: space-between">
                <h2 style="margin-top: 10px;">{file}</h2>
                <Button
                    appearance=ButtonAppearance::Subtle
                    attr:style="min-width: 0; margin-left: 5px"
                    on_click=move |_| set_selected_file.set(String::new())
                >
                    "⨉"
                </Button>
            </Space>

            <Text>"Size: 86 KB"</Text>
            <Text>"Last modified: "{format_date_time(Utc::now().naive_utc())}</Text>
            <Text>Created: {format_date_time(Utc::now().naive_utc())}</Text>
            <Space attr:style="flex: 1; gap: 5px; padding-top: 10px">
                <Button>Download</Button>
                <Button>Upload new version</Button>
            </Space>
        </Space>
    }
}
