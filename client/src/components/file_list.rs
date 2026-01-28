use crate::model::node::{DecryptedNode, NodeMetadata};
use leptos::prelude::*;
use thaw::{Button, ButtonAppearance, Text};

#[component]
pub(crate) fn FileList(
    #[prop(into)] files: Signal<Vec<DecryptedNode>>,
    selection: RwSignal<Option<DecryptedNode>>,
) -> impl IntoView {
    let on_click = move |file: DecryptedNode| {
        let selected = selection.get().clone();
        let is_selected = selected.is_some() && selected.unwrap().id == file.id;

        selection.set(if is_selected {
            None
        } else {
            Some(file.clone())
        });
    };

    view! {
        <For
            each=move || files.get()
            key=|file| file.id
            children=move |file| {
                let (file, _) = signal(file);
                view! {
                    <File
                        name=Signal::derive(move || {
                            let NodeMetadata::V1(metadata) = file.get().metadata;
                            metadata.name
                        })
                        on:click=move |_| on_click(file.get())
                    />
                }
            }

        />
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
