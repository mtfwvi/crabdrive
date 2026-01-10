use leptos::prelude::*;
use thaw::{Button, ButtonAppearance, Text};

#[component]
pub(crate) fn FileList(
    #[prop(into)] files: Signal<Vec<String>>,
    set_selected_file: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <For
            each=move || files.get()
            key=|file| file.clone()
            children=move |file| {
                view! {
                    <File name=file.clone() on:click=move |_| set_selected_file.set(file.clone()) />
                }
            }
        />
    }
}

#[component]
fn File(#[prop(into)] name: Signal<String>) -> impl IntoView {
    view! {
        <Button
            appearance=ButtonAppearance::Subtle
            attr:style="width: 100%; display: flex; justify-content: start;"
        >
            <Text>{name}</Text>
        </Button>
    }
}
