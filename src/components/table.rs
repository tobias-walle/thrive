use leptos::ev::Event;
use shared::{Coordinate, State, TableCell};

use crate::prelude::*;
use crate::tauri;

#[component]
pub fn Table(cx: Scope) -> impl IntoView {
    let (coords, _set_coords) = create_signal(
        cx,
        (0..10)
            .flat_map(|row| (0..10).map(move |col| Coordinate { col, row }))
            .collect::<Vec<_>>(),
    );
    let (state, set_state) = create_signal(cx, State::new());

    let update_cell_text = move |coord: Coordinate, cell: &TableCell, event: &Event| {
        let new_cell = TableCell {
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

    create_effect(cx, move |_| {
        log!("{:?}", state.get());
    });

    view! { cx,
        <div class="m-0 relative">
            <For
                each=move || coords.get()
                key=|coord| *coord
                view=move |cx, coord: Coordinate| {
                    let (focused, set_focused) = create_signal(cx, false);
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
                                        true => state.get().cell(&coord).text.clone(),
                                        false => state.get().cell(&coord).computed.clone(),
                                    }
                                }
                                on:input=move |event| update_cell_text(coord, state.get().cell(&coord), &event)
                                on:focus=move |_| set_focused.set(true)
                                on:blur=move |_| set_focused.set(false)
                            />
                        </div>
                }
                }
            />
        </div>
    }
}
