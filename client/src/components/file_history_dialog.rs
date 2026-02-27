use crate::api::download_file;
use crate::components::data_provider::revisions_provider::RevisionsProvider;
use crate::components::revision_list::RevisionList;
use crate::constants::DEFAULT_TOAST_TIMEOUT;
use crate::model::node::{DecryptedNode, NodeMetadata};
use crabdrive_common::storage::RevisionId;
use leptos::prelude::*;
use thaw::{
    Button, ButtonAppearance, Dialog, DialogActions, DialogBody, DialogContent, DialogSurface,
    DialogTitle, Toast, ToastIntent, ToastOptions, ToastTitle, ToasterInjection,
};

#[component]
pub(crate) fn FileHistoryDialog(
    #[prop(into)] open: RwSignal<bool>,
    on_close: Callback<()>,
    node: Signal<DecryptedNode>,
) -> impl IntoView {
    let toaster = ToasterInjection::expect_context();

    let add_toast = move |text: String| {
        toaster.dispatch_toast(
            move || {
                view! {
                    <Toast>
                        <ToastTitle>{text}</ToastTitle>
                    </Toast>
                }
            },
            ToastOptions::default()
                .with_intent(ToastIntent::Info)
                .with_timeout(DEFAULT_TOAST_TIMEOUT),
        )
    };

    let file_name = Signal::derive(move || {
        let NodeMetadata::V1(metadata) = node.get().metadata;
        metadata.name
    });

    let download_action = Action::new_local(|input: &DecryptedNode| {
        let node = input.to_owned();
        async move {
            /* TODO: Download correct revision */
            download_file(node).await.map_err(|err| err.to_string())
        }
    });
    Effect::new(move || {
        let status = download_action.value().get();
        if status.is_some() {
            let response = status.unwrap();
            if response.is_err() {
                add_toast(format!(
                    "Failed to download {}: {}",
                    file_name.get(),
                    response.err().unwrap()
                ))
            }
        }
    });
    let on_select_for_download = Callback::new(move |revision_id: RevisionId| {
        tracing::debug!("handling download for revision_id={}", revision_id);
        download_action.dispatch(node.get().clone());
    });

    view! {
        <RevisionsProvider node let:revisions>
            <Dialog open>
                <DialogSurface class="w-fit">
                    <DialogBody>
                        <DialogTitle>
                            {move || format!("Earlier versions of {}", file_name.get())}
                        </DialogTitle>
                        <DialogContent>
                            <RevisionList revisions on_select_for_download />
                        </DialogContent>
                        <DialogActions>
                            <Button
                                appearance=ButtonAppearance::Primary
                                on_click=move |_| on_close.run(())
                            >
                                "Close"
                            </Button>
                        </DialogActions>
                    </DialogBody>
                </DialogSurface>
            </Dialog>
        </RevisionsProvider>
    }
}
