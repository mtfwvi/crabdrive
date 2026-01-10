use leptos::prelude::*;
use thaw::{Breadcrumb, BreadcrumbButton, BreadcrumbDivider, BreadcrumbItem};

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
                            <BreadcrumbDivider attr:style="font-size: 1.2rem" />
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
    view! {
        <BreadcrumbItem>
            <BreadcrumbButton>
                <Show when=move || is_last.get() fallback=move || view! { <h2>{name}</h2> }>
                    <h1>{name}</h1>
                </Show>
            </BreadcrumbButton>
        </BreadcrumbItem>
    }
}
