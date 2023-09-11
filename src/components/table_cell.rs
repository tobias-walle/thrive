use leptos::ev::Event;
use shared::{Coordinate, State, TableCell as TableCellModel};

use crate::prelude::*;
use crate::tauri;

#[component]
pub fn TableCell(
    cx: Scope,
    coord: Coordinate,
    state: ReadSignal<State>,
    set_state: WriteSignal<State>,
) -> impl IntoView {
    let (focused, set_focused) = create_signal(cx, false);

    let cell = create_memo(cx, move |_| state.get().cell(&coord).clone());

    let update_cell_text = move |event: &Event| {
        let cell = cell.get_untracked();
        let new_cell = TableCellModel {
            text: event_target_value(event),
            ..cell.clone()
        };
        set_state.update(|state| {
            state.set_cell(coord, new_cell.clone());
        });
        spawn_local(async move {
            let state = state.get_untracked();
            let updated_cells = tauri::api::compute(state, coord).await;
            set_state.update(|state| {
                for cell in updated_cells {
                    state.set_cell(cell.coord, cell.cell);
                }
            });
        });
    };

    view! {
        cx,
        <div
            class="absolute outline outline-black outline-[1px] flex justify-center items-center p-[2px]"
            style:top=move || format!("{}px", coord.row * 30)
            style:height=move || format!("{}px", 29)
            style:left=move || format!("{}px", coord.col * 80)
            style:width=move || format!("{}px", 79)
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
