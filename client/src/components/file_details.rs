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
            <h2 style="margin-top: 10px">{file}</h2>
            <Text>"Size: 86 KB"</Text>
            <Text>"Last modified: "{format_date_time(Utc::now().naive_utc())}</Text>
            <Text>Created: {format_date_time(Utc::now().naive_utc())}</Text>
            <Space>
                <Button>Download</Button>
                <Button>Upload new version</Button>
            </Space>
            <Button
                appearance=ButtonAppearance::Subtle
                attr:style="position: absolute; top: 10px; right: 10px; min-width: 0"
                on_click=move |_| set_selected_file.set(String::new())
            >
                "⨉"
            </Button>
        </Space>
    }
}
