use leptos::prelude::*;
use thaw::{
    Button, ButtonAppearance, ComponentRef, Dialog, DialogActions, DialogBody, DialogContent,
    DialogSurface, DialogTitle, Input, InputRef,
};

#[component]
pub(crate) fn InputDialog(
    #[prop(into)] open: RwSignal<bool>,
    #[prop(into)] title: Signal<String>,
    #[prop(into)] placeholder: Signal<String>,
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
        <Dialog open>
            <DialogSurface class="w-fit">
                <DialogBody>
                    <DialogTitle>{title}</DialogTitle>
                    <DialogContent>
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
                            {confirm_label}
                        </Button>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    }
}
