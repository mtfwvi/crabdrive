use crate::api::create_folder;
use crate::model::node::DecryptedNode;
use leptos::prelude::*;
use thaw::{
    Button, ButtonAppearance, ComponentRef, Dialog, DialogActions, DialogBody, DialogContent,
    DialogSurface, DialogTitle, Input, InputRef, Toast, ToastIntent, ToastOptions, ToastTitle,
    ToasterInjection,
};

#[component]
pub(crate) fn FolderCreationDialog<F>(
    #[prop(into)] open: RwSignal<bool>,
    #[prop(into)] parent: Signal<DecryptedNode>,
    on_complete: F,
) -> impl IntoView
where
    F: Fn() + Send + Sync + Copy + 'static,
{
    let toaster = ToasterInjection::expect_context();

    let add_toast = move |text: String| {
        toaster.dispatch_toast(
            move || view! {<Toast><ToastTitle>{text}</ToastTitle></Toast>},
            ToastOptions::default().with_intent(ToastIntent::Info),
        )
    };

    let input_ref = ComponentRef::<InputRef>::new();
    let value = RwSignal::new(String::new());

    Effect::new(move || {
        if open.get() {
            request_animation_frame(move || input_ref.get_untracked().unwrap().focus())
        }
    });

    let creation_action = Action::new_local(move |input: &String| {
        let name = input.to_owned();
        async move { create_folder(parent.get(), name).await }
    });

    let handle_confirm = move || {
        creation_action.dispatch(value.get());
        on_complete();
        value.set(String::new());
    };

    Effect::new(move || {
        let status = creation_action.value().get();
        if status.is_some() {
            match status.unwrap() {
                Ok(folder) => {
                    add_toast(format!("Created folder successfully with id {}", folder.id))
                }
                Err(e) => add_toast(format!("Failed to create folder: {}", e)),
            }
        }
    });

    view! {
        <Dialog open>
            <DialogSurface class="w-fit">
                <DialogBody>
                    <DialogTitle>"Create new folder"</DialogTitle>
                    <DialogContent>
                        <Input
                            value
                            placeholder="Folder name"
                            comp_ref=input_ref
                            class="!border-none w-full"
                            on:keypress=move |e| {
                                if e.key() == "Enter" && !value.get().is_empty() {
                                    handle_confirm();
                                }
                            }
                        />
                    </DialogContent>

                    <DialogActions>
                        <Button
                            appearance=ButtonAppearance::Secondary
                            on_click=move |_| open.set(false)
                        >
                            "Cancel"
                        </Button>
                        <Button
                            appearance=ButtonAppearance::Primary
                            on_click=move |_| handle_confirm()
                            disabled=Signal::derive(move || value.get().is_empty())
                        >
                            "Create"
                        </Button>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    }
}
