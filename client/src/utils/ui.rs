use crate::model::node::DecryptedNode;
use chrono::NaiveDateTime;
use crabdrive_common::storage::NodeType;

pub fn format_date_time(naive_date_time: NaiveDateTime) -> String {
    naive_date_time.format("%d/%m/%Y, %H:%M:%S").to_string()
}

pub fn format_number_as_ordinal(number: usize) -> String {
    match number {
        1 => "first".to_string(),
        2 => "second".to_string(),
        3 => "third".to_string(),
        x => format!("{}th", x),
    }
}

pub fn shorten_file_name(name: String) -> String {
    let length = name.len();
    if length > 30 {
        let start = name[..18].to_string();
        let end = name[length - 10..].to_string();
        format!("{}…{}", start, end)
    } else {
        name
    }
}

pub fn get_node_icon(node_type: NodeType, name: String) -> &'static icondata_core::IconData {
    let file_extension = name.split('.').last().unwrap_or_default().to_owned();

    match node_type {
        NodeType::Folder => icondata_mdi::MdiFolderOutline,
        NodeType::Link => icondata_mdi::MdiLinkBoxOutline,
        NodeType::File => match file_extension.as_str() {
            "zip" | "7zip" | "gz" => icondata_mdi::MdiFolderZipOutline,
            "pdf" | "txt" | "md" => icondata_mdi::MdiFileDocumentOutline,
            "html" | "xml" | "json" | "toml" | "yml" | "yaml" | "rs" => {
                icondata_mdi::MdiFileCodeOutline
            }
            "png" | "jpg" | "jpeg" | "gif" | "ico" => icondata_mdi::MdiFileImageOutline,
            "mp4" | "mov" | "avi" => icondata_mdi::MdiFileVideoOutline,
            "mp3" | "wav" | "flac" => icondata_mdi::MdiFileMusicOutline,
            "doc" | "docx" | "odt" => icondata_mdi::MdiFileWordOutline,
            "xls" | "xlsx" | "ods" => icondata_mdi::MdiFileExcelOutline,
            "ppt" | "pptx" | "odp" => icondata_mdi::MdiFilePowerpointOutline,
            "csv" | "tsv" => icondata_mdi::MdiFileTableOutline,
            _ => icondata_mdi::MdiFileOutline,
        },
    }
}

pub fn get_owner_username(node: DecryptedNode) -> Option<String> {
    let owner_id = node.owner_id;

    node.has_access
        .into_iter()
        .find(|(user_id, _)| user_id == &owner_id)
        .map(|(_, username)| username)
}

pub fn get_share_acceptor_usernames(node: DecryptedNode) -> Option<Vec<String>> {
    let owner_id = node.owner_id;

    let share_acceptor_usernames: Vec<String> = node
        .has_access
        .into_iter()
        .filter(|(user_id, _)| user_id != &owner_id)
        .map(|(_, username)| username)
        .collect();

    if share_acceptor_usernames.is_empty() {
        None
    } else {
        Some(share_acceptor_usernames)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    #[test_case(2026, 1, 7, 16, 32, 1, "07/01/2026, 16:32:01")]
    #[test_case(2020, 1, 1, 0, 0, 0, "01/01/2020, 00:00:00")]
    fn test_format_date_time(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        expected: &str,
    ) {
        let naive_date_time = NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, minute, second)
            .unwrap();
        assert_eq!(format_date_time(naive_date_time), expected.to_string());
    }

    #[test_case(1, "first")]
    #[test_case(2, "second")]
    #[test_case(3, "third")]
    #[test_case(4, "4th")]
    #[test_case(10, "10th")]
    #[test_case(1234, "1234th")]
    fn test_format_number_as_ordinal(number: usize, expected: &str) {
        let expected = expected.to_owned();
        assert_eq!(format_number_as_ordinal(number), expected);
    }

    #[test_case("example.txt", "example.txt")]
    #[test_case("file_name_over_thirty_characters.md", "file_name_over_thi…racters.md")]
    #[test_case(
        "extremely_long_file_name_way_over_thirty_chars.md",
        "extremely_long_fil…y_chars.md"
    )]
    fn test_shorten_file_name(full: &str, expected: &str) {
        let full_name = full.to_owned();
        let expected = expected.to_owned();
        assert_eq!(shorten_file_name(full_name), expected);
    }
}
