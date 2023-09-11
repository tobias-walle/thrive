use leptos::ev::Event;
use shared::{Coordinate, TableCell as TableCellModel, TableState};
use shared::{FormatPixel, TableDimensions};

use crate::prelude::*;
use crate::tauri;

#[component]
pub fn TableCell(
    cx: Scope,
    coord: Coordinate,
    state: RwSignal<TableState>,
    dimensions: RwSignal<TableDimensions>,
) -> impl IntoView {
    let (focused, set_focused) = create_signal(cx, false);

    let cell = create_memo(cx, move |_| state.get().cell(&coord).clone());

    let update_cell_text = move |event: &Event| {
        let cell = cell.get_untracked();
        let new_cell = TableCellModel {
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

    view! {
        cx,
        <div
            class="absolute outline outline-black outline-[1px] flex justify-center items-center font-mono"
            style:top=move || (coord.row * dimensions.get().row_height).px()
            style:height=move || (dimensions.get().row_height - dimensions.get().border_width).px()
            style:left=move || (coord.col * dimensions.get().column_width).px()
            style:width=move || (dimensions.get().column_width - dimensions.get().border_width).px()
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
