use leptos::{
    ev::{Event, KeyboardEvent},
    html::Textarea,
};
use shared::{Coordinate, TableCell, TableState};

use crate::{
    debug,
    models::{FocusedCoordinate, SignalPair},
    prelude::*,
    tauri,
};

#[derive(Debug, Clone, Copy)]
pub struct IsCodeBarFocused(pub bool);

#[component]
pub fn CodeBar(cx: Scope) -> impl IntoView {
    let (focused_coord, set_focused_coord) =
        use_context::<SignalPair<FocusedCoordinate>>(cx).expect("Missing context");
    let (state, set_state) = use_context::<SignalPair<TableState>>(cx).expect("Missing context");
    let (_is_code_bar_focused, set_is_code_bar_focused) =
        use_context::<SignalPair<IsCodeBarFocused>>(cx).expect("Missing context");
    let focused_cell = move || {
        focused_coord
            .get()
            .coord()
            .map(|c| state.get().cell(c).clone())
    };

    let recompute_cell = move |coord: Coordinate| {
        spawn_local(async move {
            let state = state.get_untracked();
            let updated_cells = tauri::api::compute(state.clone(), coord).await;
            debug!(&updated_cells);
            set_state.update(|state| {
                for updated_cell in updated_cells {
                    match state.cell_mut(&updated_cell.coord) {
                        Some(cell_mut) => cell_mut.computed = updated_cell.cell.computed,
                        None => (),
                    };
                }
            });
        });
    };

    // TODO: Refactor out duplication with Cell Component
    let update_cell_text = move |event: &Event| {
        log!("INPUT");
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
        recompute_cell(coord);
    };

    let textarea_ref = create_node_ref::<Textarea>(cx);
    let handle_keydown = move |event: KeyboardEvent| match event.key().as_ref() {
        "Escape" => {
            let Some(textarea) = textarea_ref.get() else { return };
            textarea.blur().expect("Blur of text area failed");
            set_focused_coord.set(FocusedCoordinate(None));
        }
        _ => (),
    };

    view! {
        cx,
        <div class="flex p-2 w-[100vw] bg-slate-200 border-b border-black">
            <div class="flex-initial p-1 pr-2">𝑓</div>
            <textarea
                ref=textarea_ref
                class="flex-1 p-1 min-h-[calc(1rem+8px)] font-mono focus:outline-none border border-black"
                rows=1
                spellcheck="false"
                autocomplete="false"
                prop:value=move || focused_cell().map_or_else(String::new, |c| c.text)
                on:input=move |event| update_cell_text(&event)
                on:focus=move |_| set_is_code_bar_focused.set(IsCodeBarFocused(true))
                on:blur=move |_| set_is_code_bar_focused.set(IsCodeBarFocused(false))
                on:keydown=handle_keydown
            />
        </div>
    }
}
