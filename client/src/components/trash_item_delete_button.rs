use crate::api::delete_node_tree;
use crate::constants::{DEFAULT_TOAST_TIMEOUT, INFINITE_TOAST_TIMEOUT};
use crate::model::node::DecryptedNode;
use leptos::prelude::*;
use thaw::{Button, Toast, ToastIntent, ToastOptions, ToastTitle, ToasterInjection};

#[component]
pub(crate) fn TrashItemDeleteButton(
    node: Signal<DecryptedNode>,
    on_deleted: Callback<()>,
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

    let delete_action = Action::new_local(move |_| async move {
        delete_node_tree(node.get_untracked())
            .await
            .map_err(|err| err.to_string())
    });

    Effect::new(move || {
        let status = delete_action.value().get();
        if status.is_some() {
            match status.unwrap() {
                Ok(_) => add_toast(
                    "Deleted item successfully".to_string(),
                    ToastIntent::Success,
                ),
                Err(e) => add_toast(format!("Failed to delete item: {}", e), ToastIntent::Error),
            }
            on_deleted.run(());
        }
    });

    view! {
        <Button
            on_click=move |_| {
                delete_action.dispatch(());
            }
            icon=icondata_mdi::MdiDeleteForeverOutline
            block=true
        >
            "Delete forever"
        </Button>
    }
}
