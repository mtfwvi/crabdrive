use crate::api::{get_root_node, move_node_out_of_trash};
use crate::constants::{DEFAULT_TOAST_TIMEOUT, INFINITE_TOAST_TIMEOUT};
use crate::model::node::DecryptedNode;
use leptos::prelude::*;
use thaw::{
    Button, ButtonAppearance, Toast, ToastIntent, ToastOptions, ToastTitle, ToasterInjection,
};

#[component]
pub(crate) fn TrashItemRestoreButton(
    node: Signal<DecryptedNode>,
    on_restored: Callback<()>,
) -> impl IntoView {
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

    let restore_action = Action::new_local(move |_| async move {
        let root_node = get_root_node().await.map_err(|err| err.to_string())?;
        move_node_out_of_trash(node.get_untracked(), root_node)
            .await
            .map_err(|err| err.to_string())
    });

    Effect::new(move || {
        let status = restore_action.value().get();
        if status.is_some() {
            match status.unwrap() {
                Ok(_) => add_toast(
                    "Restored item successfully".to_string(),
                    ToastIntent::Success,
                ),
                Err(e) => add_toast(format!("Failed to restore item: {}", e), ToastIntent::Error),
            }
            on_restored.run(());
        }
    });

    view! {
        <Button
            on_click=move |_| {
                restore_action.dispatch(());
            }
            icon=icondata_mdi::MdiRestore
            appearance=ButtonAppearance::Primary
            block=true
        >
            "Restore"
        </Button>
    }
}
