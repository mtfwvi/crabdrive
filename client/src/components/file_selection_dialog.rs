use leptos::prelude::*;
use thaw::{
    Button, ButtonAppearance, Dialog, DialogActions, DialogBody, DialogContent, DialogSurface,
    DialogTitle, Divider, Text, Upload, UploadDragger,
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

    let handle_confirm = move || {
        on_confirm(selection.get().unwrap());
        selection.set(None)
    };

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
                                <Show when=move || selection.get().is_some()>
                                    <Divider class="py-4" />
                                    <Text>
                                        {move || {
                                            let length = selection.get().unwrap().length();
                                            format!(
                                                "{} {} selected",
                                                length,
                                                if length == 1 { "file" } else { "files" },
                                            )
                                        }}
                                    </Text>
                                </Show>
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
                            on_click=move |_| handle_confirm()
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
