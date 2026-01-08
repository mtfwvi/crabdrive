use crate::components::file_details::FileDetails;
use crate::components::file_list::FileList;
use crate::components::path_breadcrumb::PathBreadcrumb;
use crate::model::File;
use chrono::{NaiveDate, Utc};
use crabdrive_common::data::{DataAmount, DataUnit};
use leptos::prelude::*;
use thaw::{Layout, LayoutSider, Space};

#[component]
pub(crate) fn FolderView() -> impl IntoView {
    let example_file = File {
        name: String::from("file.md"),
        size: DataAmount::new(86f64, DataUnit::Kilobyte),
        last_modified_at: Utc::now().naive_utc(),
        created_at: NaiveDate::from_ymd_opt(2016, 7, 8)
            .unwrap()
            .and_hms_opt(9, 10, 11)
            .unwrap(),
    };

    view! {
        <Layout
            attr:style="padding: 10px 50px; width: 80%; box-shadow: none; outline: 1px solid lightgray"
            has_sider=true
        >
            <Space
                vertical=true
                attr:style="flex-direction: column; gap: 10px; width: 70%; padding: 20px"
            >
                <h1 style="margin-bottom: 0; margin-left: 5px">Foldername</h1>
                <PathBreadcrumb _elements=vec!["home", "jona", "Music"] />
                <FileList />
            </Space>

            <LayoutSider attr:style="border-left: 1px dotted lightgray; padding: 20px">
                <FileDetails file=example_file />
            </LayoutSider>
        </Layout>
    }
}
