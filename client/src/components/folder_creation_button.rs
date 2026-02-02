use crate::components::folder_creation_dialog::FolderCreationDialog;
use crate::model::node::DecryptedNode;
use leptos::prelude::*;
use thaw::Button;

#[component]
pub(crate) fn FolderCreationButton(
    #[prop(into)] parent_node: Signal<DecryptedNode>,
    on_created: Callback<()>,
) -> impl IntoView {
    let folder_creation_dialog_open = RwSignal::new(false);

    view! {
        <Button
            on_click=move |_| folder_creation_dialog_open.set(true)
            icon=icondata::MdiFolderPlusOutline
        >
            "Create folder"
        </Button>

        <FolderCreationDialog // TODO Move logic to this component - make dialog something like InputDialog?
            open=folder_creation_dialog_open
            parent=parent_node
            on_complete=move || {
                folder_creation_dialog_open.set(false);
                on_created.run(());
            }
        />
    }
}
