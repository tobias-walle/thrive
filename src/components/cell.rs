use crate::models::Rectangle;
use crate::models::{FormatPixel, TableDimensions};
use leptos::ev::Event;
use shared::{Coordinate, TableCell, TableState};

use crate::prelude::*;
use crate::tauri;

#[component]
pub fn Cell(
    cx: Scope,
    coord: Coordinate,
    state: RwSignal<TableState>,
    dimensions: RwSignal<TableDimensions>,
) -> impl IntoView {
    let (focused, set_focused) = create_signal(cx, false);

    let cell = create_memo(cx, move |_| state.get().cell(&coord).clone());

    let update_cell_text = move |event: &Event| {
        let cell = cell.get_untracked();
        let new_cell = TableCell {
            text: event_target_value(event),
            ..cell.clone()
        };
        state.update(|state| {
            state.set_cell(coord, new_cell.clone());
        });
        spawn_local(async move {
            let updated_cells = tauri::api::compute(state.get_untracked(), coord).await;
            state.update(|state| {
                for cell in updated_cells {
                    state.set_cell(cell.coord, cell.cell);
                }
            });
        });
    };

    let rect = move || {
        let d = dimensions.get();
        Rectangle {
            top: coord.row * d.row_height + d.border_width,
            height: d.row_height - d.border_width,
            left: coord.col * d.column_width + d.border_width,
            width: d.column_width - d.border_width,
        }
    };

    view! {
        cx,
        <div
            class="absolute flex justify-center items-center font-mono"
            style:top=move || rect().top.px()
            style:height=move || rect().height.px()
            style:left=move || rect().left.px()
            style:width=move || rect().width.px()
            title=move || format!("{}|{}", coord.row, coord.col)
        >
            <input
                class="w-full h-full p-1 focus:outline focus:outline-cyan-500 focus:outline-[2px] rounded-none"
                spellcheck="false"
                autocomplete="false"
                prop:value=move || {
                    match focused.get() {
                        true => cell.get().text.clone(),
                        false => cell.get().computed.clone(),
                    }
                }
                on:input=move |event| update_cell_text(&event)
                on:focus=move |_| set_focused.set(true)
                on:blur=move |_| set_focused.set(false)
            />
        </div>
    }
}
