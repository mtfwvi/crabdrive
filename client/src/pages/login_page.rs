use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use std::time::Duration;
use thaw::{
    Button, ButtonAppearance, Image, Input, InputType, Space, SpaceAlign, Text, Toast, ToastBody,
    ToastIntent, ToastOptions, ToasterInjection,
};

#[component]
pub(crate) fn LoginPage(register_new_account: bool) -> impl IntoView {
    let navigate = use_navigate();
    let toaster = ToasterInjection::expect_context();

    let add_toast = move |text: String| {
        toaster.dispatch_toast(
            move || {
                view! {
                    <Toast>
                        <ToastBody>{text}</ToastBody>
                    </Toast>
                }
            },
            ToastOptions::default()
                .with_intent(ToastIntent::Info)
                .with_timeout(Duration::from_millis(10_000)),
        )
    };

    let username = RwSignal::new(String::from(""));
    let password = RwSignal::new(String::from(""));
    let navigate_register = navigate.clone();
    let navigate_to_register =
        Callback::new(move |_| navigate_register("/register", Default::default()));

    let navigate_submit = navigate.clone();
    let on_submit = Callback::new(move |_| {
        let username = username.get();
        let password = password.get();
        if username.is_empty() || password.is_empty() {
            return;
        }

        add_toast(format!("Received login info: ({},{})", username, password));
        navigate_submit("/00000000-0000-0000-0000-000000000000", Default::default())
    });

    view! {
        <Space vertical=true class="h-screen py-15" align=SpaceAlign::Center>
            <Space align=SpaceAlign::Center>
                <Image src="/logo.svg" attr:width=50 />
                <Text class="!text-3xl !font-bold">"crabdrive"</Text>
            </Space>
            <Text class="!text-lg !font-bold">"Rust native cloud storage"</Text>

            <form
                class="h-fit w-100 mt-15 px-15 py-10 flex flex-col gap-2 rounded-sm outline outline-gray-300"
                on:submit=move |e| {
                    e.prevent_default();
                    on_submit.run(())
                }
            >
                <Text class="!text-2xl">
                    {move || if register_new_account { "Register new account" } else { "Login" }}
                </Text>
                <Input placeholder="Username" class="w-full" autofocus=true value=username />
                <Input
                    placeholder="Password"
                    class="w-full"
                    input_type=InputType::Password
                    value=password
                />
                <Button appearance=ButtonAppearance::Primary block=true>
                    {move || if register_new_account { "Register" } else { "Login" }}
                </Button>

                <Show when=move || !register_new_account>
                    <Button
                        appearance=ButtonAppearance::Transparent
                        block=true
                        on_click=move |_| navigate_to_register.run(())
                    >
                        "Have no account yet? Register here"
                    </Button>
                </Show>
            </form>
        </Space>
    }
}
