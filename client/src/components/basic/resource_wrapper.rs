use anyhow::Result;
use leptos::prelude::*;
use std::fmt::Debug;
use thaw::{Spinner, Text};

#[component]
pub(crate) fn ResourceWrapper<T, F, V>(
    resource: LocalResource<Result<T, String>>,
    children: F,
    #[prop(into)] error_text: Signal<String>,
) -> impl IntoView
where
    T: Clone + Debug + Send + Sync + 'static,
    F: Fn(Signal<T>) -> V + Send + Sync + 'static,
    V: IntoView + 'static,
{
    let render_error =
        move |e| view! { <Text>{format!("{}: {:?}", error_text.get(), e)}</Text> }.into_any();

    view! {
        <Transition fallback=move || {
            view! { <Spinner class="m-auto" /> }
        }>
            {move || {
                resource
                    .get()
                    .map(|result| {
                        match result {
                            Ok(value) => children(Signal::derive(move || value.clone())).into_any(),
                            Err(e) => render_error(e),
                        }
                    })
            }}
        </Transition>
    }
}
