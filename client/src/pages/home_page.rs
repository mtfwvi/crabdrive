use crate::components::account_sider::AccountSider;
use crate::components::content_frame::{ContentFrame, ContentViewType};
use crate::utils::auth::is_authenticated;
use crate::utils::browser::SessionStorage;
use crabdrive_common::storage::NodeId;
use crabdrive_common::uuid::UUID;
use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params_map};
use thaw::{Button, ButtonAppearance, Layout, LayoutSider, ToasterInjection};

#[derive(PartialEq, Clone, Copy)]
pub(crate) enum HomePageType {
    Folder,
    Shared,
    Trash,
}

#[component]
pub(crate) fn HomePage(#[prop(into)] view_type: Signal<HomePageType>) -> impl IntoView {
    let node_id: Signal<Option<NodeId>> = Signal::derive(move || {
        let parameter = use_params_map().get().get("id")?;
        UUID::parse_string(&parameter)
    });

    let toaster = ToasterInjection::expect_context();
    let navigate = use_navigate();

    let _redirect_if_not_logged_in = Effect::new(move || {
        let is_logged_in = is_authenticated().unwrap_or_default();
        if !is_logged_in {
            navigate("/login", Default::default())
        }
    });

    view! {
        <Layout content_style="padding: 30px 40px; height: 100vh" has_sider=true>
            <LayoutSider class="!min-w-73">
                <AccountSider />
            </LayoutSider>

            <Layout
                class="h-full flex-1 rounded-sm outline outline-gray-300"
                content_style="height: 100%"
                has_sider=true
            >
                <ContentFrame content_type=Signal::derive(move || {
                    match view_type.get() {
                        HomePageType::Folder => {
                            let node_id = node_id
                                .get()
                                .unwrap_or_else(|| {
                                    let root_id: Option<NodeId> = SessionStorage::get("root_id")
                                        .unwrap_or_default();
                                    root_id.unwrap_or_else(NodeId::nil)
                                });
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
