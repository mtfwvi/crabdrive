use leptos::prelude::*;
use std::fmt::Debug;
use thaw::{Spinner, Text};

#[component]
pub(crate) fn ResourceWrapper<T, F>(
    resource: LocalResource<Result<T, String>>,
    render: F,
    #[prop(into)] error_text: Signal<String>,
    #[prop(optional, default = true)] fallback_spinner: bool,
) -> impl IntoView
where
    T: Clone + Debug + 'static,
    F: Fn(T) -> AnyView + Send + Sync + 'static,
{
    let render_error = move |e| {
        view! {
            <Text>{format!("{}: {}", error_text.get(), e)}</Text>
        }
        .into_any()
    };

    view! {
        <Suspense fallback=move || {
            view! {
                <Show when=move || fallback_spinner>
                    <Spinner />
                </Show>
            }
        }>
            {move || {
                resource
                    .get()
                    .map(|result| {
                        match result {
                            Ok(value) => render(value),
                            Err(e) => render_error(e)
                        }
                    })
            }}
        </Suspense>
    }
}
