use leptos::prelude::*;
use std::fmt::Debug;
use thaw::{Spinner, Text};

#[component]
pub(crate) fn ResourceWrapper<T, F>(
    resource: LocalResource<Result<T, String>>,
    render: F,
    error_text: &'static str,
    #[prop(optional, default = true)] fallback_spinner: bool,
) -> impl IntoView
where
    T: Clone + Debug + 'static,
    F: Fn(T) -> AnyView + Send + Sync + 'static,
{
    tracing::debug!("{:?}", resource.get_untracked());
    tracing::debug!("{:?}", error_text);
    tracing::debug!("{:?}", fallback_spinner);
    view! {
        <Suspense fallback=move || {view! { <Show when=move || fallback_spinner><Spinner /></Show> }}>
            {move || {
                resource
                    .get()
                    .map(|result| {
                        match result {
                            Ok(value) => render(value),
                            Err(e) => {
                                view! {
                                    <Text>
                                        {format!(
                                            "{}: {}",
                                            error_text,
                                            e,
                                        )}
                                    </Text>
                                }
                                    .into_any()
                            }
                        }
                    })
            }}
        </Suspense>
    }
}
