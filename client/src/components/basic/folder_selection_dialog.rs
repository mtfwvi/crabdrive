use crate::components::data_provider::children_provider::ChildrenProvider;
use crate::components::data_provider::path_provider::PathProvider;
use crate::components::node_list::NodeList;
use crate::components::path_breadcrumb::PathBreadcrumb;
use crabdrive_common::storage::NodeId;
use leptos::prelude::*;
use thaw::{
    Button, ButtonAppearance, Dialog, DialogActions, DialogBody, DialogContent, DialogSurface,
    DialogTitle, Divider,
};

#[component]
pub(crate) fn FolderSelectionDialog(
    #[prop(into)] open: RwSignal<bool>,
    on_confirm: Callback<NodeId>,
    #[prop(into)] title: Signal<String>,
    #[prop(into)] confirm_label: String,
    start_folder: Signal<NodeId>,
) -> impl IntoView {
    let currently_open = RwSignal::new_local(start_folder.get_untracked());

    Effect::new(move || {
        if open.get() {
            currently_open.set(start_folder.get_untracked())
        }
    });

    view! {
        // TODO: Extract dialog component to reduce indentation
        <Dialog open>
            <DialogSurface class="w-fit">
                <DialogBody>
                    <DialogTitle>{title}</DialogTitle>
                    <DialogContent>
                        <div class="min-h-32 mt-2 p-6 rounded-sm outline outline-gray-300">
                            <PathProvider
                                node_id=Signal::derive(move || currently_open.get())
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
                    </DialogContent>
                    <DialogActions>
                        <Button
                            appearance=ButtonAppearance::Secondary
                            on_click=move |_| open.set(false)
                        >
                            "Cancel"
                        </Button>
                        <Button
                            appearance=ButtonAppearance::Primary
                            on_click=move |_| on_confirm.run(currently_open.get())
                            disabled=Signal::derive(move || {
                                start_folder.get() == currently_open.get()
                            })
                        >
                            {confirm_label}
                        </Button>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    }
}
