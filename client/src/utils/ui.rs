use chrono::NaiveDateTime;

pub(crate) fn format_date_time(naive_date_time: NaiveDateTime) -> String {
    naive_date_time.format("%d/%m/%Y, %H:%M:%S").to_string()
}

pub(crate) fn format_number_as_ordinal(number: usize) -> String {
    match number {
        1 => "first".to_string(),
        2 => "second".to_string(),
        3 => "third".to_string(),
        x => format!("{}th", x),
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::ui::format_date_time;
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
}
