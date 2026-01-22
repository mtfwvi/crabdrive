use leptos::prelude::*;
use thaw::{
    Breadcrumb, BreadcrumbButton, BreadcrumbDivider, BreadcrumbItem, Text, Toast, ToastIntent,
    ToastOptions, ToastTitle, ToasterInjection,
};

#[component]
pub(crate) fn PathBreadcrumb(#[prop(into)] node_names: Signal<Vec<String>>) -> impl IntoView {
    view! {
        <Breadcrumb>
            <ForEnumerate
                each=move || node_names.get()
                key=|name| name.clone()
                children=move |index, name| {
                    let is_not_last = move || index.get() != node_names.get().len() - 1;

                    view! {
                        <PathBreadcrumbItem name=name is_last=!is_not_last() />
                        <Show when=is_not_last>
                            <BreadcrumbDivider class="!text-xl" />
                        </Show>
                    }
                }
            />
        </Breadcrumb>
    }
}

#[component]
fn PathBreadcrumbItem(
    #[prop(into)] name: Signal<String>,
    #[prop(optional, into)] is_last: Signal<bool>,
) -> impl IntoView {
    let toaster = ToasterInjection::expect_context();

    let add_toast = move |_| {
        toaster.dispatch_toast(
            move || {
                view! {
                    <Toast>
                        <ToastTitle>"TODO"</ToastTitle>
                    </Toast>
                }
            },
            ToastOptions::default().with_intent(ToastIntent::Error),
        )
    };

    view! {
        <BreadcrumbItem>
            <BreadcrumbButton on:click=add_toast>
                <Text class=format!(
                    "!{} !font-bold",
                    if is_last.get() { "text-3xl" } else { "text-2xl" },
                )>{name}</Text>
            </BreadcrumbButton>
        </BreadcrumbItem>
    }
}

#[cfg(test)]
mod tests {
    use crate::components::path_breadcrumb::PathBreadcrumb;
    use leptos::prelude::*;
    use thaw::{ConfigProvider, ToasterProvider};
    use wasm_bindgen_test::__rt::wasm_bindgen::JsCast;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    async fn test_path_breadcrumb() {
        let document = document();
        let test_wrapper = document.create_element("section").unwrap();
        let _ = document.body().unwrap().append_child(&test_wrapper);

        let _dispose = mount_to(test_wrapper.clone().unchecked_into(), || {
            view! {
                <ConfigProvider>
                    <ToasterProvider>
                        <PathBreadcrumb node_names=RwSignal::new(
                            vec![String::from("superfolder"), String::from("subfolder")],
                        ) />
                    </ToasterProvider>
                </ConfigProvider>
            }
        });

        assert!(test_wrapper.outer_html().contains("superfolder"));
        assert!(test_wrapper.outer_html().contains("subfolder"));
    }
}
