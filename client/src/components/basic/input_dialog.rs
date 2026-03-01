use crate::components::basic::custom_dialog::CustomDialog;
use leptos::prelude::*;
use thaw::{ComponentRef, Input, InputRef};

#[component]
pub(crate) fn InputDialog(
    #[prop(into)] open: RwSignal<bool>,
    #[prop(into)] title: Signal<String>,
    #[prop(into, optional)] placeholder: Signal<String>,
    #[prop(into)] confirm_label: Signal<String>,
    on_confirm: Callback<String>,
) -> impl IntoView {
    let input_ref = ComponentRef::<InputRef>::new();
    let value = RwSignal::new(String::new());

    Effect::new(move || {
        if open.get() {
            request_animation_frame(move || input_ref.get_untracked().unwrap().focus())
        }
    });

    let handle_confirm = move || {
        on_confirm.run(value.get());
        value.set(String::new());
    };

    view! {
        <CustomDialog
            open
            title
            show_cancel=true
            show_confirm=true
            confirm_label
            confirm_disabled=Signal::derive(move || value.get().is_empty())
            on_confirm=handle_confirm
        >
            <Input
                value
                placeholder
                comp_ref=input_ref
                class="!border-none w-full"
                on:keypress=move |e| {
                    if e.key() == "Enter" && !value.get().is_empty() {
                        handle_confirm();
                    }
                }
            />
        </CustomDialog>
    }
}
