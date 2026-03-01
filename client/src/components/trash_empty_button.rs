use crate::api::empty_trash;
use crate::constants::{DEFAULT_TOAST_TIMEOUT, INFINITE_TOAST_TIMEOUT};
use leptos::prelude::*;
use thaw::{
    Button, ButtonAppearance, Toast, ToastIntent, ToastOptions, ToastTitle, ToasterInjection,
};

// TODO: Extract these kinds of buttons to component with only success and error toast + action content
#[component]
pub fn TrashEmptyButton(on_emptied: Callback<()>) -> impl IntoView {
    let toaster = ToasterInjection::expect_context();

    let add_toast = move |text: String, intent: ToastIntent| {
        toaster.dispatch_toast(
            move || {
                view! {
                    <Toast>
                        <ToastTitle>{text}</ToastTitle>
                    </Toast>
                }
            },
            ToastOptions::default().with_intent(intent).with_timeout(
                if matches!(intent, ToastIntent::Error) {
                    INFINITE_TOAST_TIMEOUT
                } else {
                    DEFAULT_TOAST_TIMEOUT
                },
            ),
        )
    };

    let empty_action =
        Action::new_local(|_| async move { empty_trash().await.map_err(|err| err.to_string()) });

    Effect::new(move || {
        let status = empty_action.value().get();
        if status.is_some() {
            match status.unwrap() {
                Ok(_) => add_toast("Emptied trash".to_string(), ToastIntent::Success),
                Err(e) => add_toast(format!("Failed to empty trash: {}", e), ToastIntent::Error),
            }
            on_emptied.run(());
        }
    });

    view! {
        <Button
            on_click=move |_| {
                empty_action.dispatch(());
            }
            appearance=ButtonAppearance::Secondary
            icon=icondata_mdi::MdiDeleteEmptyOutline
            block=true
        >
            "Empty trash"
        </Button>
    }
}
