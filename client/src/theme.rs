use std::collections::HashMap;
use thaw::Theme;

pub fn get_theme(dark: bool) -> Theme {
    let brand_colors: HashMap<i32, &str> = HashMap::from([
        (10, "#060201"),
        (20, "#25110F"),
        (30, "#401818"),
        (40, "#561D1E"),
        (50, "#6E2125"),
        (60, "#86252C"),
        (70, "#9E2833"),
        (80, "#B82B3A"),
        (90, "#D22D41"),
        (100, "#E2414E"),
        (110, "#E95D60"),
        (120, "#EF7474"),
        (130, "#F58B88"),
        (140, "#F9A09C"),
        (150, "#FCB5B1"),
        (160, "#FFCAC6"),
    ]);

    if dark {
        Theme::custom_dark(&brand_colors)
    } else {
        Theme::custom_light(&brand_colors)
    }
}
