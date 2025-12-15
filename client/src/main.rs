use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <h1>crabdrive</h1>
            <p>Rust native cloud storage</p>
        }
    })
}
