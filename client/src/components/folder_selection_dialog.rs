use crate::api::{get_children, path_to_root};
use crate::components::basic::resource_wrapper::ResourceWrapper;
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
    current_node: Signal<NodeId>,
) -> impl IntoView {
    let currently_open = RwSignal::new_local(current_node.get_untracked());
    let path_res = LocalResource::new(move || async move {
        let node_id = currently_open.get();
        path_to_root(node_id).await.map_err(|err| err.to_string())
    });

    Effect::new(move || {
        if open.get() {
            currently_open.set(current_node.get())
        }
    });

    let set_open = Callback::new(move |node_id| {
        currently_open.set(node_id);
        path_res.refetch();
    });

    let handle_confirm = move || on_confirm.run(currently_open.get());

    view! {
        <Dialog open>
            <DialogSurface class="w-fit">
                <DialogBody>
                    <DialogTitle>{title}</DialogTitle>
                    <DialogContent>
                        <div class="min-h-32 mt-2 p-6 rounded-sm outline outline-gray-300">
                            <ResourceWrapper
                                resource=path_res
                                error_text=Signal::derive(move || {
                                    format!(
                                        "The path to node '{}' could not be loaded from the server",
                                        currently_open.get(),
                                    )
                                })
                                fallback_spinner=false
                                children=move |path| {
                                    let current_node = Signal::derive(move || {
                                        path.get().last().expect("Failed due to empty path").clone()
                                    });
                                    let children_res = LocalResource::new(move || {
                                        let current_node = current_node.get();
                                        async move {
                                            get_children(current_node)
                                                .await
                                                .map_err(|err| err.to_string())
                                        }
                                    });

                                    view! {
                                        <ResourceWrapper
                                            resource=children_res
                                            error_text=Signal::derive(move || {
                                                format!(
                                                    "The children of '{}' could not be loaded from the server",
                                                    currently_open.get(),
                                                )
                                            })
                                            let:children
                                        >
                                            <PathBreadcrumb path on_select=set_open compact=true />
                                            <Divider class="mb-2" />
                                            <NodeList
                                                nodes=children
                                                on_node_click=Callback::new(move |_| {})
                                                on_folder_dblclick=set_open
                                                folders_only=true
                                            />
                                        </ResourceWrapper>
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
                            on_click=move |_| handle_confirm()
                            disabled=Signal::derive(move || {
                                current_node.get() == currently_open.get()
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
