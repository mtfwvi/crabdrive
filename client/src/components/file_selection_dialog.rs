use leptos::prelude::*;
use thaw::{
    Button, ButtonAppearance, Dialog, DialogActions, DialogBody, DialogContent, DialogSurface,
    DialogTitle, Upload, UploadDragger,
};
use web_sys::FileList;

#[component]
pub(crate) fn FileSelectionDialog<F, S>(
    #[prop(into)] open: RwSignal<bool>,
    on_confirm: S,
    title: F,
    #[prop(optional, default = true)] allow_multiple: bool,
) -> impl IntoView
where
    F: Fn() -> String + Send + Sync + 'static,
    S: Fn(FileList) + Send + Sync + 'static,
{
    let selection = RwSignal::new_local(None);

    let custom_request = move |file_list: FileList| selection.set(Some(file_list));

    view! {
        <Dialog open>
            <DialogSurface class="w-fit">
                <DialogBody>
                    <DialogTitle>{title}</DialogTitle>
                    <DialogContent>
                        <Upload custom_request multiple=allow_multiple>
                            // This inline style is necessary since this div that needs styling
                            // is inserted between Upload and UploadDragger in the DOM
                            <style>".thaw-upload__trigger { width: 100% } "</style>

                            <UploadDragger>
                                // TODO: Display list of selected files
                                "Click or drag a file to this area"
                            </UploadDragger>
                        </Upload>
                    </DialogContent>
                    <DialogActions>
                        <Button
                            appearance=ButtonAppearance::Secondary
                            on_click=move |_| open.set(false)
                        >
                            Cancel
                        </Button>
                        <Button
                            appearance=ButtonAppearance::Primary
                            on_click=move |_| on_confirm(selection.get().unwrap())
                            disabled=Signal::derive(move || selection.get().is_none())
                        >
                            "Upload"
                        </Button>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    }
}
