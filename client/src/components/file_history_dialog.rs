use crate::api::download_file;
use crate::components::basic::custom_dialog::CustomDialog;
use crate::components::data_provider::revisions_provider::RevisionsProvider;
use crate::components::revision_list::RevisionList;
use crate::constants::{DEFAULT_TOAST_TIMEOUT, INFINITE_TOAST_TIMEOUT};
use crate::model::node::{DecryptedNode, NodeMetadata};
use crabdrive_common::storage::FileRevision;
use crabdrive_common::uuid::UUID;
use leptos::prelude::*;
use thaw::{
    Spinner, SpinnerSize, Toast, ToastIntent, ToastOptions, ToastTitle, ToastTitleMedia,
    ToasterInjection,
};

#[component]
pub(crate) fn FileHistoryDialog(
    #[prop(into)] open: RwSignal<bool>,
    node: Signal<DecryptedNode>,
) -> impl IntoView {
    let toaster = ToasterInjection::expect_context();
    let download_in_progress_toast_id = UUID::random();
    let add_download_in_progress_toast = move || {
        toaster.dispatch_toast(
            move || {
                view! {
                    <Toast>
                        <ToastTitle>
                            "Download started..." <ToastTitleMedia slot>
                                <Spinner size=SpinnerSize::Tiny />
                            </ToastTitleMedia>
                        </ToastTitle>
                    </Toast>
                }
            },
            ToastOptions::default()
                .with_id(download_in_progress_toast_id.into())
                .with_intent(ToastIntent::Info)
                .with_timeout(INFINITE_TOAST_TIMEOUT),
        )
    };
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

    let file_name = Signal::derive(move || {
        let NodeMetadata::V1(metadata) = node.get().metadata;
        metadata.name
    });

    let download_action = Action::new_local(move |input: &(DecryptedNode, FileRevision)| {
        let (node, revision) = input.to_owned();

        add_download_in_progress_toast();

        async move {
            download_file(node, Some(revision))
                .await
                .map_err(|err| err.to_string())
        }
    });
    Effect::new(move || {
        let status = download_action.value().get();
        if status.is_some() {
            toaster.dismiss_toast(download_in_progress_toast_id.into());
            match status.unwrap() {
                Ok(_) => add_toast("Download complete".to_string(), ToastIntent::Success),
                Err(e) => add_toast(
                    format!("Failed to download {}: {}", file_name.get(), e),
                    ToastIntent::Error,
                ),
            }
        }
    });
    let on_select_for_download = Callback::new(move |revision: FileRevision| {
        tracing::debug!("handling download for revision_id={}", revision.id);
        download_action.dispatch((node.get_untracked().clone(), revision));
    });

    view! {
        <RevisionsProvider node let:revisions>
            <CustomDialog
                open
                title=Signal::derive(move || format!("Earlier versions of {}", file_name.get()))
                show_cancel=true
                show_confirm=false
            >
                <RevisionList revisions on_select_for_download />
            </CustomDialog>
        </RevisionsProvider>
    }
}
