use leptos::ev::Event;
use shared::{TableCell, TableState};

use crate::{
    models::{FocusedCoordinate, SignalPair},
    prelude::*,
    tauri,
};

#[component]
pub fn CodeBar(cx: Scope) -> impl IntoView {
    let (focused_coord, _) =
        use_context::<SignalPair<FocusedCoordinate>>(cx).expect("Missing context");
    let (state, set_state) = use_context::<SignalPair<TableState>>(cx).expect("Missing context");
    let focused_cell = move || {
        focused_coord
            .get()
            .coord()
            .map(|c| state.get().cell(c).clone())
    };

    // TODO: Refactor out duplication with Cell Component
    let update_cell_text = move |event: &Event| {
        let coord = match focused_coord.get().coord() {
            Some(coord) => *coord,
            None => return,
        };
        let cell = state.get().cell(&coord).clone();
        let new_cell = TableCell {
            text: event_target_value(event),
            ..cell.clone()
        };
        set_state.update(|state| {
            state.set_cell(coord, new_cell.clone());
        });
        spawn_local(async move {
            let updated_cells = tauri::api::compute(state.get_untracked(), coord).await;
            set_state.update(|state| {
                for cell in updated_cells {
                    state.set_cell(cell.coord, cell.cell);
                }
            });
        });
    };

    view! {
        cx,
        <div class="flex p-2 w-[100vw] bg-slate-200 border-b border-black">
            <div class="flex-initial p-1 pr-2">𝑓</div>
            <textarea
                class="flex-1 p-1 min-h-[calc(1rem+8px)] font-mono focus:outline-none border border-black"
                rows=1
                spellcheck="false"
                autocomplete="false"
                prop:value=move || focused_cell().map_or_else(String::new, |c| c.text)
                on:input=move |event| update_cell_text(&event)
            />
        </div>
    }
}
