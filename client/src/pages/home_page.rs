use crate::api::auth::logout;
use crate::components::content_frame::{ContentFrame, ContentViewType};
use crate::constants::DEFAULT_TOAST_TIMEOUT;
use crate::utils::auth::is_authenticated;
use crate::utils::browser::SessionStorage;
use crabdrive_common::storage::NodeId;
use crabdrive_common::uuid::UUID;
use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params_map};
use thaw::{
    Button, ButtonAppearance, ButtonGroup, Divider, Flex, FlexAlign, Image, Layout, LayoutSider,
    Space, SpaceAlign, Text, Toast, ToastIntent, ToastOptions, ToastTitle, ToasterInjection,
};

#[derive(PartialEq, Clone, Copy)]
pub(crate) enum HomePageType {
    Folder,
    Shared,
    Trash,
}

// TODO: Extract stuff out of this - make smaller
#[component]
pub(crate) fn HomePage(#[prop(into)] view_type: Signal<HomePageType>) -> impl IntoView {
    let node_id: Signal<Option<NodeId>> = Signal::derive(move || {
        let parameter = use_params_map().get().get("id")?;
        UUID::parse_string(&parameter)
    });

    let navigate = use_navigate();
    let navigate_to = navigate.clone();
    let navigate_to = Callback::new(move |path| navigate_to(path, Default::default()));

    let navigate_to_node = Callback::new(move |node_id: NodeId| {
        navigate(&format!("/{}", node_id), Default::default())
    });

    Effect::new(move || {
        let is_logged_in = is_authenticated().unwrap_or_default();
        if !is_logged_in {
            navigate_to.run("/login");
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
                Ok(_) => navigate_to.run("/login"),
                Err(e) => add_toast(format!("Logout failed: {}", e)),
            }
        }
    });

    let on_go_to_root = move || {
        let node_id: Option<UUID> = SessionStorage::get("root_id").unwrap_or_default();
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
                    <ButtonGroup class="w-full" vertical=true>
                        <Button
                            on_click=move |_| on_go_to_root()
                            icon=icondata_mdi::MdiFolderHomeOutline
                            class="flex-1"
                        >
                            "Root"
                        </Button>
                        <Button
                            on_click=move |_| navigate_to.run("/shared")
                            icon=icondata_mdi::MdiFolderAccountOutline
                            class="flex-1"
                        >
                            "Shared"
                        </Button>
                        <Button
                            on_click=move |_| navigate_to.run("/trash")
                            icon=icondata_mdi::MdiTrashCanOutline
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
                        icon=icondata_mdi::MdiLogout
                        class="mt-3"
                    >
                        {move || format!("Log out ({})", username.get())}
                    </Button>
                </Flex>
            </LayoutSider>

            <Layout
                class="h-full flex-1 rounded-sm outline outline-gray-300"
                content_style="height: 100%"
                has_sider=true
            >
                <ContentFrame content_type=Signal::derive(move || {
                    match view_type.get() {
                        HomePageType::Folder => {
                            let node_id = node_id.get().unwrap();
                            ContentViewType::Folder(node_id)
                        }
                        HomePageType::Shared => ContentViewType::Shared,
                        HomePageType::Trash => ContentViewType::Trash,
                    }
                }) />
            </Layout>
        </Layout>
        <Button
            class="absolute bottom-[15px] left-[20px] !text-gray-500 hover:!text-[var(--colorNeutralForeground2BrandHover)]"
            appearance=ButtonAppearance::Transparent
            on_click=move |_| {
                toaster.dismiss_all();
            }
        >
            "Dismiss all"
        </Button>
    }
}
