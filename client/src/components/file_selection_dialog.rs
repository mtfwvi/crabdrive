use leptos::prelude::*;
use thaw::{
    Button, ButtonAppearance, Dialog, DialogActions, DialogBody, DialogContent, DialogSurface,
    DialogTitle, Divider, Space, Text, Upload, UploadDragger,
};
use web_sys::{File, FileList};

#[component]
pub(crate) fn FileSelectionDialog<F>(
    #[prop(into)] open: RwSignal<bool>,
    on_confirm: F,
    #[prop(into)] title: Signal<String>,
    #[prop(optional, default = true)] allow_multiple: bool,
) -> impl IntoView
where
    F: Fn(FileList) + Send + Sync + 'static,
{
    let selection = RwSignal::new_local(None);

    let set_file_list = move |file_list: FileList| selection.set(Some(file_list));

    let selected_files = move || {
        if selection.get().is_none() {
            return vec![];
        }
        let current_selection = selection.get().unwrap();

        let mut files: Vec<File> = vec![];
        for i in 0..current_selection.length() {
            let file = current_selection.item(i).unwrap();
            files.push(file)
        }
        files
    };

    let file_names = move || {
        let names: Vec<String> = selected_files().iter().map(File::name).collect();
        names
    };

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
                        <Upload custom_request=set_file_list multiple=allow_multiple>
                            // This inline style is necessary since this div that needs styling
                            // is inserted between Upload and UploadDragger in the DOM
                            <style>".thaw-upload__trigger { width: 100% } "</style>

                            <UploadDragger>
                                "Click or drag a file to this area"
                                <Show when=move || selection.get().is_some()>
                                    <Space vertical=true>
                                        <Divider class="py-4" />
                                        <Text>
                                            {move || {
                                                let length = selection.get().unwrap().length();
                                                format!(
                                                    "{} {} selected:",
                                                    length,
                                                    if length == 1 { "file" } else { "files" },
                                                )
                                            }}
                                        </Text>
                                        <For
                                            each=file_names
                                            key=|name| name.clone()
                                            let:name
                                        >
                                            <Text class="!block !text-center">{name}</Text>
                                        </For>
                                    </Space>
                                </Show>
                            </UploadDragger>
                        </Upload>
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
