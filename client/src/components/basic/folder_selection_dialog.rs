use crate::components::basic::custom_dialog::CustomDialog;
use crate::components::data_provider::children_provider::ChildrenProvider;
use crate::components::data_provider::path_provider::PathProvider;
use crate::components::node_list::NodeList;
use crate::components::path_breadcrumb::PathBreadcrumb;
use crate::model::node::DecryptedNode;
use leptos::prelude::*;
use thaw::Divider;

#[component]
pub(crate) fn FolderSelectionDialog(
    #[prop(into)] open: RwSignal<bool>,
    on_confirm: Callback<DecryptedNode>,
    #[prop(into)] title: Signal<String>,
    #[prop(into)] confirm_label: String,
    start_folder: Signal<DecryptedNode>,
) -> impl IntoView {
    let currently_open = RwSignal::new_local(start_folder.get_untracked());

    Effect::new(move || {
        if open.get() {
            currently_open.set(start_folder.get_untracked())
        }
    });

    view! {
        <CustomDialog
            open
            title
            show_cancel=true
            show_confirm=true
            confirm_label
            confirm_disabled=Signal::derive(move || start_folder.get() == currently_open.get())
            on_confirm=Callback::new(move |_| on_confirm.run(currently_open.get()))
        >
            <div class="min-h-32 mt-2 p-6 rounded-sm outline outline-gray-300">
                <PathProvider
                    node_id=Signal::derive(move || currently_open.get().id)
                    children=move |path, refetch_path| {
                        let set_open = Callback::new(move |node_id| {
                            currently_open.set(node_id);
                            refetch_path.run(());
                        });

                        view! {
                            <ChildrenProvider
                                node=Signal::derive(move || {
                                    path.get().last().expect("Failed due to empty path").clone()
                                })
                                let:children
                                let:_refetch_children
                            >
                                <PathBreadcrumb path on_select=set_open compact=true />
                                <Divider class="mb-2" />
                                <NodeList
                                    nodes=children
                                    on_node_click=Callback::new(move |_| {})
                                    on_folder_dblclick=set_open
                                    folders_only=true
                                />
                            </ChildrenProvider>
                        }
                    }
                />
            </div>
        </CustomDialog>
    }
}
