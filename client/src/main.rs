pub(crate) mod components;
pub(crate) mod pages;

use leptos::prelude::*;
use pages::demo_page::DemoPage;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(DemoPage)
}
