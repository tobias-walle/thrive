use crate::components::IsCodeBarFocused;
use crate::models::{FocusedCoordinate, FormatPixel, TableDimensions};
use crate::models::{Rectangle, SignalPair};
use leptos::ev::{Event, MouseEvent};
use leptos::html::Textarea;
use shared::{Coordinate, TableCell, TableState};

use crate::tauri;
use crate::{debug, prelude::*};

#[component]
pub fn Cell(cx: Scope, coord: Coordinate) -> impl IntoView {
    let (state, set_state) = use_context::<SignalPair<TableState>>(cx).expect("Missing context");
    let (dimensions, _) = use_context::<SignalPair<TableDimensions>>(cx).expect("Missing context");
    let (focused_coord, set_focused_coord) =
        use_context::<SignalPair<FocusedCoordinate>>(cx).expect("Missing context");

    let (is_directly_focused, set_is_directly_focused) = create_signal(cx, false);
    let is_focused = move || focused_coord.get().is_focused(&coord);

    let cell = create_memo(cx, move |_| state.get().cell(&coord).clone());

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

    let update_cell = move |new_cell: TableCell| {
        set_state.update(|state| {
            state.set_cell(coord, new_cell.clone());
        });
        recompute_cell(coord);
    };

    let update_cell_text = move |event: &Event| {
        update_cell(TableCell {
            text: event_target_value(event),
            ..cell.get_untracked().clone()
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

    let (is_code_bar_focused, _) =
        use_context::<SignalPair<IsCodeBarFocused>>(cx).expect("Missing context");
    let handle_cell_click = move |event: MouseEvent| {
        if is_code_bar_focused.get().0 {
            let focused_coord = focused_coord.get();
            let Some(focused_coord) = focused_coord.coord() else { return };
            set_state.update(|state| {
                let focused_cell = state.cell(focused_coord).clone();
                let text = focused_cell.text;
                state.set_cell(
                    *focused_coord,
                    TableCell {
                        text: format!(r#"{text}c("{coord}")"#),
                        ..focused_cell
                    },
                );
            });
            recompute_cell(*focused_coord);
            event.prevent_default();
        }
    };

    view! {
        cx,
        <div
            class="absolute flex justify-center items-center font-mono"
            class=("bg-slate-100", is_focused)
            style:top=move || rect().top.px()
            style:height=move || rect().height.px()
            style:left=move || rect().left.px()
            style:width=move || rect().width.px()
            title=move || cell.get().computed
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
                on:mousedown=handle_cell_click
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
