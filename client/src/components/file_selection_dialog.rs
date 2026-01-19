use leptos::prelude::*;
use thaw::{
    Button, ButtonAppearance, Dialog, DialogActions, DialogBody, DialogContent, DialogSurface,
    DialogTitle, Space, Upload, UploadDragger,
};
use web_sys::FileList;

#[component]
pub(crate) fn FileSelectionDialog<F, S>(
    #[prop(into)] open: RwSignal<bool>,
    on_select: S,
    title: F,
    #[prop(into)] button_label: Signal<String>,
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
            <DialogSurface>
                <DialogBody>
                    <DialogTitle>{title}</DialogTitle>
                    <DialogContent>
                        <Space vertical=true>
                            <Show when=move || allow_multiple>Multiple files may be selected.</Show>
                            <Upload custom_request multiple=allow_multiple>
                                <UploadDragger>"Click or drag a file to this area"</UploadDragger>
                            // TODO: Display list of selected files
                            </Upload>
                        </Space>
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
                            on_click=move |_| on_select(selection.get().unwrap())
                            disabled=Signal::derive(move || selection.get().is_none())
                        >
                            {button_label}
                        </Button>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    }
}
