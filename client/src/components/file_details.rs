use crate::model::File;
use leptos::prelude::*;
use thaw::{Space, Text};

#[component]
pub(crate) fn FileDetails(#[prop(into)] file: File) -> impl IntoView {
    view! {
        <Space vertical=true>
            <h2>{file.name}</h2>
            <Text>Size : {file.size.to_string()}</Text>
            <Text>Last modified: {file.last_modified_at.to_string()}</Text>
            <Text>Created: {file.created_at.to_string()}</Text>
        </Space>
    }
}
