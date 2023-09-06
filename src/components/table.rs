use shared::{Coordinate, TableCell};
use std::collections::HashMap;

use leptos::{ev::Event, *};

use crate::{debug, tauri};

#[component]
pub fn Table(cx: Scope) -> impl IntoView {
    let (coords, _set_coords) = create_signal(
        cx,
        (0..10)
            .flat_map(|row| (0..10).map(move |col| Coordinate { row, col }))
            .collect::<Vec<_>>(),
    );
    let (cells, set_cells) = create_signal(cx, HashMap::<Coordinate, TableCell>::new());

    let get_cell = move |coord: &Coordinate| match cells.get().get(coord) {
        Some(cell) => cell.to_owned(),
        None => TableCell {
            coord: coord.to_owned(),
            ..Default::default()
        },
    };

    let update_cell_text = move |coord: Coordinate, cell: &TableCell, event: &Event| {
        let new_cell = TableCell {
            text: event_target_value(event),
            ..cell.clone()
        };
        set_cells.update(|cells| {
            cells.insert(coord, new_cell.clone());
        });
        spawn_local(async move {
            let new_cell_with_computed = tauri::api::compute(new_cell).await;
            set_cells.update(|cells| {
                cells.insert(coord, new_cell_with_computed);
            });
        });
    };

    create_effect(cx, move |_| {
        log!("{:?}", cells.get());
    });

    view! { cx,
        <div class="m-0 relative">
            <For
                each=move || coords.get()
                key=|coord| *coord
                view=move |cx, coord: Coordinate| {
                    let cell = get_cell(&coord);
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
                                prop:value=move || {
                                    let cell = get_cell(&coord);
                                    match focused.get() {
                                        true => cell.text.clone(),
                                        false => cell.computed.clone(),
                                    }
                                }
                                on:input=move |event| update_cell_text(coord, &cell, &event)
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
