use leptos::prelude::*;
use thaw::{
    Breadcrumb, BreadcrumbButton, BreadcrumbDivider, BreadcrumbItem, Popover, PopoverTrigger, Text,
};

#[component]
pub(crate) fn PathBreadcrumb(_elements: Vec<&'static str>) -> impl IntoView {
    view! {
        <Breadcrumb>

            <PathBreadcrumbItem name="home" />
            <BreadcrumbDivider />
            <PathBreadcrumbItem name="jonathan" />
            <BreadcrumbDivider />
            <PathBreadcrumbItem name="Documents" />
        </Breadcrumb>
    }
}

#[component]
fn PathBreadcrumbItem(name: &'static str) -> impl IntoView {
    view! {
        <BreadcrumbItem>
            <Popover>
                <PopoverTrigger slot>
                    <BreadcrumbButton>
                        <Text>{name}</Text>
                    </BreadcrumbButton>
                </PopoverTrigger>
                "TODO: Open folder on click"
            </Popover>
        </BreadcrumbItem>
    }
}
