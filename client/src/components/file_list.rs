use crate::model::node::{DecryptedNode, NodeMetadata};
use leptos::prelude::*;
use thaw::{Button, ButtonAppearance, Text};

#[component]
pub(crate) fn FileList(
    #[prop(into)] files: Signal<Vec<DecryptedNode>>,
    selection: RwSignal<Option<DecryptedNode>>,
) -> impl IntoView {
    view! {
        <For
            each=move || files.get()
            key=|file| file.id
            children=move |file| {
                let file_name = Signal::derive(move || {
                    let NodeMetadata::V1(metadata) = selection.get().unwrap().metadata;
                    metadata.name
                });
                // `selection.get()` cannot be None, because FileDetails is wrapped by Show

                view! {
                    <File
                        name=file_name
                        on:click=move |_| {
                            if selection.get().unwrap() == file {
                                selection.set(None)
                            } else {
                                selection.set(Some(file.clone()))
                            }
                        }
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
