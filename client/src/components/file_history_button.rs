use crate::components::file_history_dialog::FileHistoryDialog;
use crate::model::node::DecryptedNode;
use leptos::prelude::*;
use thaw::{Button, ButtonAppearance};

#[component]
pub fn FileHistoryButton(#[prop(into)] node: Signal<DecryptedNode>) -> impl IntoView {
    let file_history_dialog_open = RwSignal::new(false);

    view! {
        <Button
            on_click=move |_| file_history_dialog_open.set(true)
            appearance=ButtonAppearance::Secondary
            icon=icondata_mdi::MdiHistory
            block=true
        >
            "History"
        </Button>
        <FileHistoryDialog open=file_history_dialog_open node />
    }
}
