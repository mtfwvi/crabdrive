use crate::utils::ui::format_date_time;
use crabdrive_common::storage::{FileRevision, RevisionId};
use leptos::prelude::*;
use thaw::{Button, ButtonAppearance, ButtonSize, Space, SpaceGap, Text};

#[component]
pub(crate) fn RevisionList(
    revisions: Signal<Vec<FileRevision>>,
    on_select_for_download: Callback<RevisionId>,
) -> impl IntoView {
    view! {
        <Space vertical=true gap=SpaceGap::Large>
            <Text class="!text-lg">by time of creation</Text>
            <For each=move || revisions.get() key=|revision| revision.id let:revision>
                <Button
                    appearance=ButtonAppearance::Secondary
                    size=ButtonSize::Large
                    class="mb-2"
                    block=true
                    icon=icondata_mdi::MdiDownload
                    on_click=move |_| on_select_for_download.run(revision.id)
                >
                    {move || format_date_time(revision.upload_started_on)}
                </Button>
            </For>
        </Space>
    }
}
