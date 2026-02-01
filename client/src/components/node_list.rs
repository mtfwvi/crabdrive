use crate::model::node::{DecryptedNode, NodeMetadata};
use crabdrive_common::storage::NodeType;
use leptos::prelude::*;
use thaw::{Button, ButtonAppearance, Flex, FlexGap, FlexJustify, Text};

#[component]
pub(crate) fn NodeList(
    #[prop(into)] nodes: Signal<Vec<DecryptedNode>>,
    on_select: Callback<DecryptedNode>,
) -> impl IntoView {
    let on_click = move |node: DecryptedNode| on_select.run(node);

    let sorted_nodes = move |node_type: NodeType| {
        let all_nodes = nodes.get();
        let mut filtered_nodes: Vec<DecryptedNode> = all_nodes
            .into_iter()
            .filter(|node| node.node_type == node_type)
            .collect();

        filtered_nodes.sort_by_key(|node| {
            let NodeMetadata::V1(metadata) = node.metadata.clone();
            metadata.name
        });

        filtered_nodes
    };

    view! {
        <Show
            when=move || !nodes.get().is_empty()
            fallback=|| view! { <Text>"Folder is empty"</Text> }
        >
            <Flex vertical=true gap=FlexGap::Large justify=FlexJustify::SpaceBetween>
                <For
                    each=move || vec![NodeType::Folder, NodeType::File, NodeType::Link]
                    key=|node_type| *node_type
                    let:node_type
                >
                    // For grouping each node type's nodes together
                    <div class=if sorted_nodes(node_type).is_empty() {"hidden"} else {""}>
                        <For
                            each=move || sorted_nodes(node_type)
                            key=|node| node.id
                            children=move |node| {
                                let node = Signal::derive(move || node.clone());
                                view! {
                                    <NodeListItem
                                        name=Signal::derive(move || {
                                            let NodeMetadata::V1(metadata) = node.get().metadata;
                                            metadata.name
                                        })
                                        node_type=Signal::derive(move || node.get().node_type)
                                        on:click=move |_| on_click(node.get())
                                    />
                                }
                            }
                        />
                    </div>
                </For>
            </Flex>
        </Show>
    }
}

#[component]
fn NodeListItem(
    #[prop(into)] name: Signal<String>,
    #[prop(into)] node_type: Signal<NodeType>,
) -> impl IntoView {
    let file_extension = name.get().split('.').last().unwrap_or_default().to_owned();
    let file_icon = move || match file_extension.as_str() {
        "zip" | "7zip" | "gz" => icondata::MdiFolderZipOutline,
        "pdf" | "txt" | "md" => icondata::MdiFileDocumentOutline,
        "html" | "xml" | "json" | "toml" | "yml" | "yaml" | "rs" => icondata::MdiFileCodeOutline,
        "png" | "jpg" | "jpeg" | "gif" | "ico" => icondata::MdiFileImageOutline,
        "mp4" | "mov" | "avi" => icondata::MdiFileVideoOutline,
        "mp3" | "wav" | "flac" => icondata::MdiFileMusicOutline,
        "doc" | "docx" | "odt" => icondata::MdiFileWordOutline,
        "xls" | "xlsx" | "ods" => icondata::MdiFileExcelOutline,
        "ppt" | "pptx" | "odp" => icondata::MdiFilePowerpointOutline,
        "csv" | "tsv" => icondata::MdiFileTableOutline,
        _ => icondata::MdiFileOutline,
    };

    view! {
        <Button
            appearance=ButtonAppearance::Subtle
            icon=Signal::derive(move || match node_type.get() {
                NodeType::Folder => icondata::MdiFolderOutline,
                NodeType::File => file_icon(),
                NodeType::Link => icondata::MdiLinkBoxOutline,
            })
            class="w-full flex !justify-start"
        >
            <Text>{name}</Text>
        </Button>
    }
}
