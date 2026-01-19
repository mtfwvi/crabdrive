use leptos::prelude::*;
use thaw::{Button, ButtonAppearance, Text};

#[component]
pub(crate) fn FileList(
    #[prop(into)] files: Signal<Vec<String>>,
    selection: RwSignal<String>,
) -> impl IntoView {
    view! {
        <For each=move || files.get() key=|file| file.clone() let:file>
            <File
                name=file.clone()
                on:click=move |_| {
                    if selection.get() == file {
                        selection.set(String::new())
                    } else {
                        selection.set(file.clone())
                    }
                }
            />
        </For>
    }
}

#[component]
fn File(#[prop(into)] name: Signal<String>) -> impl IntoView {
    view! {
        <Button appearance=ButtonAppearance::Subtle class="w-full flex !justify-start">
            <Text>{name}</Text>
        </Button>
    }
}
