use std::fmt::Display;

use shared::Coordinate;
use shared::{FormatPixel, TableDimensions};

use crate::prelude::*;

#[component]
pub fn Border(
    cx: Scope,
    direction: BorderDirection,
    coord: Coordinate,
    dimensions: RwSignal<TableDimensions>,
) -> impl IntoView {
    let rect = move || {
        let d = dimensions.get();
        match direction {
            BorderDirection::Top => Rectangle {
                top: coord.row * d.row_height - d.border_width,
                height: d.border_width,
                left: coord.col * d.column_width,
                width: d.column_width,
            },
            BorderDirection::Left => Rectangle {
                top: coord.row * d.row_height,
                height: d.row_height,
                left: coord.col * d.column_width - d.border_width,
                width: d.border_width,
            },
        }
    };

    view! {
        cx,
        <div
            class="absolute bg-black"
            style:top=move || rect().top.px()
            style:height=move || rect().height.px()
            style:left=move || rect().left.px()
            style:width=move || rect().width.px()
            title=move || format!("B {}: {}|{}", direction, coord.row, coord.col)
        />
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BorderDirection {
    Top,
    Left,
}

impl Display for BorderDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            BorderDirection::Top => "T",
            BorderDirection::Left => "L",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug)]
struct Rectangle {
    top: i64,
    height: i64,
    left: i64,
    width: i64,
}
