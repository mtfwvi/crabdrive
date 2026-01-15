use std::collections::HashMap;
use thaw::Theme;

pub(crate) fn get_theme() -> Theme {
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

    Theme::custom_light(&brand_colors)
}
