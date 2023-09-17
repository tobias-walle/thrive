use crate::models::{FocusedCoordinate, FormatPixel, TableDimensions};
use crate::models::{Rectangle, SignalPair};
use leptos::ev::Event;
use leptos::html::Textarea;
use shared::{Coordinate, TableCell, TableState};

use crate::prelude::*;
use crate::tauri;

#[component]
pub fn Cell(cx: Scope, coord: Coordinate) -> impl IntoView {
    let (state, set_state) = use_context::<SignalPair<TableState>>(cx).expect("Missing context");
    let (dimensions, _) = use_context::<SignalPair<TableDimensions>>(cx).expect("Missing context");
    let (focused_coord, set_focused_coord) =
        use_context::<SignalPair<FocusedCoordinate>>(cx).expect("Missing context");

    let (is_directly_focused, set_is_directly_focused) = create_signal(cx, false);
    let is_focused = move || focused_coord.get().is_focused(&coord);

    let cell = create_memo(cx, move |_| state.get().cell(&coord).clone());

    let update_cell_text = move |event: &Event| {
        let cell = cell.get_untracked();
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

    let rect = move || {
        let d = dimensions.get();
        Rectangle {
            top: coord.row * d.row_height,
            height: d.row_height,
            left: coord.col * d.column_width,
            width: d.column_width,
        }
    };

    let textarea_ref = create_node_ref::<Textarea>(cx);
    create_effect(cx, move |_| {
        let _ = cell.get();
        let Some(textarea) = textarea_ref.get() else { return };
        if !is_directly_focused.get() {
            textarea.set_scroll_top(textarea.scroll_height());
        }
    });

    view! {
        cx,
        <div
            class="absolute flex justify-center items-center font-mono"
            class=("bg-slate-100", is_focused)
            style:top=move || rect().top.px()
            style:height=move || rect().height.px()
            style:left=move || rect().left.px()
            style:width=move || rect().width.px()
            title=move || format!("{}|{}", coord.row, coord.col)
        >
            <textarea
                ref=textarea_ref
                class="w-full h-full p-1 focus:outline-none rounded-none bg-transparent resize-none"
                rows=1
                spellcheck="false"
                autocomplete="false"
                prop:value=move || {
                    match is_focused() {
                        true => cell.get().text.clone(),
                        false => cell.get().computed.clone(),
                    }
                }
                on:input=move |event| update_cell_text(&event)
                on:focus=move |_| {
                    set_is_directly_focused.set(true);
                    set_focused_coord.set(FocusedCoordinate(Some(coord)));
                }
                on:blur=move |_| {
                    set_is_directly_focused.set(false);
                }
            />
        </div>
    }
}
