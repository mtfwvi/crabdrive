use crate::api::download_file;
use crate::components::basic::custom_dialog::CustomDialog;
use crate::components::data_provider::revisions_provider::RevisionsProvider;
use crate::components::revision_list::RevisionList;
use crate::constants::DEFAULT_TOAST_TIMEOUT;
use crate::model::node::{DecryptedNode, NodeMetadata};
use crabdrive_common::storage::FileRevision;
use leptos::prelude::*;
use thaw::{Toast, ToastIntent, ToastOptions, ToastTitle, ToasterInjection};

#[component]
pub(crate) fn FileHistoryDialog(
    #[prop(into)] open: RwSignal<bool>,
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

    let download_action = Action::new_local(|input: &(DecryptedNode, FileRevision)| {
        let (node, revision) = input.to_owned();
        async move {
            download_file(node, Some(revision))
                .await
                .map_err(|err| err.to_string())
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
