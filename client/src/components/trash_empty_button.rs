use crate::constants::DEFAULT_TOAST_TIMEOUT;
use leptos::prelude::*;
use thaw::{
    Button, ButtonAppearance, Toast, ToastIntent, ToastOptions, ToastTitle, ToasterInjection,
};

// TODO: Extract these kinds of buttons to component with only success and error toast + action content
#[component]
pub(crate) fn TrashEmptyButton() -> impl IntoView {
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
            ToastOptions::default()
                .with_intent(intent)
                .with_timeout(DEFAULT_TOAST_TIMEOUT),
        )
    };

    let empty_action = Action::new_local(|_| async move { Ok(()) as Result<(), String> });
    let handle_download = move |_| {
        empty_action.dispatch(());
    };

    Effect::new(move || {
        let status = empty_action.value().get();
        if status.is_some() {
            add_toast("TODO".to_string(), ToastIntent::Info)
            // match status.unwrap() {
            //     Ok(_) => add_toast("Emptied trash".to_string(), ToastIntent::Success),
            //     Err(e) => add_toast(
            //         format!("Failed to empty trash: {}", e),
            //         ToastIntent::Error,
            //     )
            // }
        }
    });

    view! {
        <Button
            on_click=handle_download
            appearance=ButtonAppearance::Secondary
            icon=icondata_mdi::MdiDeleteEmptyOutline
            block=true
        >
            "Empty trash"
        </Button>
    }
}
