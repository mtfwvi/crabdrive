use leptos::prelude::*;
use thaw::{Card, Text, Tree, TreeItem, TreeItemLayout, TreeItemType};

#[component]
pub(crate) fn FileTree() -> impl IntoView {
    view! {
        <Card>
            <h2>"Demo filetree"</h2>
            <Tree>
                <FileTreeNode>"README.md"</FileTreeNode>
                <FileTreeNode>"Cargo.toml"</FileTreeNode>
                <TreeItem item_type=TreeItemType::Branch>
                    <FileTreeNodeLabel>src</FileTreeNodeLabel>
                    <Tree>
                        <FileTreeNode>"util.rs"</FileTreeNode>
                        <FileTreeNode>"main.rs"</FileTreeNode>
                    </Tree>
                </TreeItem>
            </Tree>
        </Card>
    }
}

#[component]
pub(crate) fn FileTreeNode(children: Children) -> impl IntoView {
    view! {
        <TreeItem item_type=TreeItemType::Leaf>
            <FileTreeNodeLabel>
                {children()}
            </FileTreeNodeLabel>
        </TreeItem>
    }
}

#[component]
pub(crate) fn FileTreeNodeLabel(children: Children) -> impl IntoView {
    println!("in fileTreeNodeLabel");
    view! {
        <TreeItemLayout>
            <Text>{children()}</Text>
        </TreeItemLayout>
    }
}
