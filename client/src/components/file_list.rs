use leptos::prelude::*;
use thaw::{Button, ButtonAppearance, Space, Text};

#[component]
pub(crate) fn FileList() -> impl IntoView {
    view! {
        <Space vertical=true>
            <File name="test.md" />
            <File name="audio.mp3" />
            <File name="document.pdf" />
        </Space>
    }
}

#[component]
fn File(name: &'static str) -> impl IntoView {
    view! {
        <Button
            appearance=ButtonAppearance::Subtle
            attr:style="width: 100%; display: flex; justify-content: start;"
        >
            <Text>{name}</Text>
        </Button>
    }
}
