use leptos::prelude::*;
use thaw::{
    Button, ButtonAppearance, Dialog, DialogActions, DialogBody, DialogContent, DialogSurface,
    DialogTitle,
};

#[derive(Clone)]
pub struct CustomDialogAction {
    label: String,
    button_appearance: ButtonAppearance,
    disabled: bool,
    on_click: Callback<()>,
}

impl CustomDialogAction {
    pub fn new(
        label: String,
        button_appearance: ButtonAppearance,
        disabled: bool,
        on_click: Callback<()>,
    ) -> Self {
        Self {
            label,
            button_appearance,
            disabled,
            on_click,
        }
    }
}

#[component]
pub(crate) fn CustomDialog(
    #[prop(into)] open: RwSignal<bool>,
    #[prop(into)] title: Signal<String>,
    #[prop(into)] show_cancel: Signal<bool>,
    #[prop(into)] show_confirm: Signal<bool>,
    #[prop(into, optional)] confirm_label: Signal<Option<String>>,
    #[prop(into, optional)] confirm_disabled: Signal<Option<bool>>,
    #[prop(into, optional)] on_confirm: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    view! {
        <Dialog open>
            <DialogSurface class="w-fit">
                <DialogBody>
                    <DialogTitle>{title}</DialogTitle>
                    <DialogContent>{children()}</DialogContent>
                    <DialogActions>
                        <Show when=move || show_cancel.get()>
                            <Button
                                appearance=ButtonAppearance::Secondary
                                on_click=move |_| open.set(false)
                            >
                                "Cancel"
                            </Button>
                        </Show>
                        <Show when=move || show_confirm.get()>
                            <Button
                                appearance=ButtonAppearance::Primary
                                on_click=move |_| on_confirm.unwrap().run(())
                                disabled=Signal::derive(move || confirm_disabled.get().unwrap())
                            >
                                {move || confirm_label.get().unwrap()}
                            </Button>
                        </Show>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    }
}
