use crate::api::auth::logout;
use crate::components::folder_view::FolderView;
use crate::constants::DEFAULT_TOAST_TIMEOUT;
use crate::utils::auth::is_authenticated;
use crate::utils::browser::SessionStorage;
use crabdrive_common::storage::NodeId;
use crabdrive_common::uuid::UUID;
use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params_map};
use thaw::{
    Button, ButtonGroup, Divider, Flex, FlexAlign, Image, Layout, LayoutSider, Space, SpaceAlign,
    Text, Toast, ToastIntent, ToastOptions, ToastTitle, ToasterInjection,
};

#[component]
pub(crate) fn HomePage() -> impl IntoView {
    let node_id: Signal<Option<NodeId>> = Signal::derive(move || {
        let parameter = use_params_map().get().get("id")?;
        UUID::parse_string(parameter)
    });

    let navigate = use_navigate();
    let navigate_to_login = navigate.clone();
    let navigate_to_login = Callback::new(move |_| navigate_to_login("/login", Default::default()));

    let navigate_to_node = Callback::new(move |node_id: NodeId| {
        navigate(&format!("/{}", node_id), Default::default())
    });

    Effect::new(move || {
        let is_logged_in = is_authenticated().unwrap_or_default();
        if !is_logged_in {
            navigate_to_login.run(());
        }
    });

    let toaster = ToasterInjection::expect_context();
    let add_toast = move |text: String| {
        toaster.dispatch_toast(
            move || {
                view! {
                    <Toast>
                        <ToastTitle>{text}</ToastTitle>
                    </Toast>
                }
            },
            ToastOptions::default()
                .with_intent(ToastIntent::Error)
                .with_timeout(DEFAULT_TOAST_TIMEOUT),
        )
    };

    let logout_action =
        Action::new_local(
            move |_: &()| async move { logout().await.map_err(|err| err.to_string()) },
        );

    Effect::new(move || {
        let status = logout_action.value().get();
        if status.is_some() {
            match status.unwrap() {
                Ok(_) => navigate_to_login.run(()),
                Err(e) => add_toast(format!("Logout failed: {}", e)),
            }
        }
    });

    let on_go_to_node = move |storage_field: &'static str| {
        let node_id: Option<UUID> = SessionStorage::get(storage_field).unwrap_or_default();
        if let Some(node_id) = node_id {
            navigate_to_node.run(node_id);
        }
    };

    let username = Signal::derive(move || {
        let storage_result = SessionStorage::get("username").unwrap_or_default();
        storage_result.unwrap_or(String::from("current user"))
    });

    view! {
        <Layout content_style="padding: 30px 40px; height: 100vh" has_sider=true>
            <LayoutSider class="!min-w-73">
                <Flex vertical=true class="w-fit" align=FlexAlign::Start>
                    <Space align=SpaceAlign::Center>
                        <Image src="/logo.svg" attr:width=50 />
                        <Text class="!text-3xl !font-bold">"crabdrive"</Text>
                    </Space>
                    <Text class="!text-lg !font-bold">"Rust native cloud storage"</Text>
                    <Divider class="mt-2 mb-4" />
                    <ButtonGroup class="w-full">
                        <Button
                            on_click=move |_| on_go_to_node("root_id")
                            icon=icondata::MdiFolderStarOutline
                            class="flex-1"
                        >
                            "Root"
                        </Button>
                        <Button
                            on_click=move |_| on_go_to_node("trash_id")
                            icon=icondata::MdiTrashCanOutline
                            class="flex-1"
                        >
                            Trash
                        </Button>
                    </ButtonGroup>
                    <Button
                        on_click=move |_| {
                            logout_action.dispatch(());
                        }
                        block=true
                        icon=icondata::MdiLogout
                    >
                        {move || format!("Log out ({})", username.get())}
                    </Button>
                </Flex>
            </LayoutSider>

            <Layout
                class="h-fit min-h-57 flex-1 rounded-sm outline outline-gray-300"
                has_sider=true
            >
                <Show
                    when=move || node_id.get().is_some()
                    fallback=|| view! { <Text>No node selected.</Text> }
                >
                    <FolderView
                        node_id=Signal::derive(move || node_id.get().unwrap())
                        is_trash=Signal::derive(move || {
                            let trash_id: Option<UUID> = SessionStorage::get("trash_id")
                                .unwrap_or_default();
                            trash_id == node_id.get()
                        })
                    />
                </Show>
            </Layout>
        </Layout>
    }
}
