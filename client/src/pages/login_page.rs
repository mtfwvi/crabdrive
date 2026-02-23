use crate::api::auth::{login, register};
use crate::constants::{DEFAULT_TOAST_TIMEOUT, INFINITE_TOAST_TIMEOUT};
use crate::utils::auth::is_valid_password;
use crabdrive_common::uuid::UUID;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use thaw::{
    Button, ButtonAppearance, ComponentRef, Image, Input, InputRef, InputType, MessageBar,
    MessageBarBody, MessageBarIntent, MessageBarLayout, MessageBarTitle, Space, SpaceAlign,
    Spinner, SpinnerSize, Text, Toast, ToastIntent, ToastOptions, ToastTitle, ToastTitleMedia,
    ToasterInjection,
};

#[derive(PartialEq, Clone, Copy)]
pub(crate) enum LoginType {
    Register,
    Login,
}

#[component]
pub(crate) fn LoginPage(#[prop(into)] login_type: Signal<LoginType>) -> impl IntoView {
    let navigate = use_navigate();
    let navigate_to_register = navigate.clone();
    let navigate_to_register =
        Callback::new(move |_| navigate_to_register("/register", Default::default()));
    let navigate_to_login = Callback::new(move |_| navigate("/login", Default::default()));

    let username_input_ref = ComponentRef::<InputRef>::new();

    let toaster = ToasterInjection::expect_context();
    let auth_in_progress_toast_id = UUID::random();
    let add_auth_in_progress_toast = move |operation: &'static str| {
        toaster.dispatch_toast(
            move || {
                view! {
                    <Toast>
                        <ToastTitle>
                            {move || {
                                format!("{} in progress, this can take a moment...", operation)
                            }} <ToastTitleMedia slot>
                                <Spinner size=SpinnerSize::Tiny />
                            </ToastTitleMedia>
                        </ToastTitle>
                    </Toast>
                }
            },
            ToastOptions::default()
                .with_id(auth_in_progress_toast_id.into())
                .with_intent(ToastIntent::Info)
                .with_timeout(INFINITE_TOAST_TIMEOUT),
        )
    };
    let add_toast = move |text: String, intent: ToastIntent| {
        toaster.dispatch_toast(
            move || {
                view! {
                    <Toast>
                        <ToastTitle>{text}</ToastTitle>
                    </Toast>
                }
            },
            ToastOptions::default()
                .with_intent(intent)
                .with_timeout(DEFAULT_TOAST_TIMEOUT),
        )
    };

    let username = RwSignal::new(String::from(""));
    let password = RwSignal::new(String::from(""));
    let is_password_valid = RwSignal::new(true);

    let register_action = Action::new_local(move |input: &(String, String)| {
        let (username, password) = input.to_owned();
        async move {
            add_auth_in_progress_toast("Registration");
            register(&username, &password)
                .await
                .map_err(|err| err.to_string())
        }
    });

    Effect::new(move || {
        let status = register_action.value().get();
        if status.is_some() {
            toaster.dismiss_toast(auth_in_progress_toast_id.into());
            match status.unwrap() {
                Ok(_) => {
                    add_toast(
                        "Registration successful, log in now!".to_string(),
                        ToastIntent::Success,
                    );
                    navigate_to_login.run(())
                }
                Err(e) => add_toast(format!("Registration failed: {}", e), ToastIntent::Error),
            }
        }
    });

    Effect::watch(
        move || login_type,
        move |_, _, _| {
            request_animation_frame(move || username_input_ref.get_untracked().unwrap().focus())
        },
        true,
    );

    let login_action = Action::new_local(move |input: &(String, String)| {
        let (username, password) = input.to_owned();
        async move {
            add_auth_in_progress_toast("Login");
            login(&username, &password, true)
                .await
                .map_err(|err| err.to_string())
        }
    });

    Effect::new(move || {
        let status = login_action.value().get();
        if status.is_some() {
            toaster.dismiss_toast(auth_in_progress_toast_id.into());
            match status.unwrap() {
                Ok(_) => {} // Login redirects instead of returning on success
                Err(e) => add_toast(format!("Login failed: {}", e), ToastIntent::Error),
            }
        }
    });

    let on_submit = Callback::new(move |_| {
        let username = username.get();
        let password = password.get();
        let password_valid = is_valid_password(&password);
        is_password_valid.set(password_valid);
        if username.is_empty() || !password_valid {
            return;
        }

        match login_type.get() {
            LoginType::Register => register_action.dispatch((username, password)),
            LoginType::Login => login_action.dispatch((username, password)),
        };
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
                attr:accept-charset="utf-8"
                on:submit=move |e: web_sys::SubmitEvent| {
                    e.prevent_default();
                    on_submit.run(())
                }
            >
                <Text class="!text-2xl">
                    {move || match login_type.get() {
                        LoginType::Register => "Register new account",
                        LoginType::Login => "Login",
                    }}
                </Text>
                <Input
                    placeholder="Username"
                    comp_ref=username_input_ref
                    class="w-full"
                    autofocus=true
                    value=username
                    autocomplete="username"
                />
                <Input
                    placeholder="Password"
                    class="w-full"
                    input_type=InputType::Password
                    value=password
                    autocomplete=match login_type.get() {
                        LoginType::Register => "new-password",
                        LoginType::Login => "current-password",
                    }
                />
                <Show when=move || {
                    !is_password_valid.get() && (login_type.get() == LoginType::Register)
                }>
                    <MessageBar intent=MessageBarIntent::Error layout=MessageBarLayout::Multiline>
                        <MessageBarBody class="mb-2">
                            <MessageBarTitle>"Invalid password"</MessageBarTitle>
                            "Must be at least 12 characters long"
                        </MessageBarBody>
                    </MessageBar>
                </Show>
                <Button appearance=ButtonAppearance::Primary block=true>
                    {move || match login_type.get() {
                        LoginType::Register => "Register",
                        LoginType::Login => "Login",
                    }}
                </Button>

                <Button
                    appearance=ButtonAppearance::Transparent
                    block=true
                    on_click=move |e: web_sys::MouseEvent| {
                        e.prevent_default();
                        let navigate = match login_type.get() {
                            LoginType::Register => navigate_to_login,
                            LoginType::Login => navigate_to_register,
                        };
                        navigate.run(())
                    }
                >
                    {move || {
                        match login_type.get() {
                            LoginType::Register => "Already have an account? Login now",
                            LoginType::Login => "Have no account yet? Register now",
                        }
                    }}
                </Button>
            </form>
        </Space>
    }
}
