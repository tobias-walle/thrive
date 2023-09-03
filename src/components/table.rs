use std::collections::HashMap;

use leptos::{ev::Event, *};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
struct Coordinate {
    row: i64,
    col: i64,
}

#[derive(Debug, Clone, Default)]
struct Cell {
    coord: Coordinate,
    text: String,
}

#[component]
pub fn Table(cx: Scope) -> impl IntoView {
    let (coords, set_coords) = create_signal(
        cx,
        (0..100)
            .flat_map(|row| (0..100).map(move |col| Coordinate { row, col }))
            .collect::<Vec<_>>(),
    );
    let (cells, set_cells) = create_signal(cx, HashMap::<Coordinate, Cell>::new());

    let get_cell = move |coord: &Coordinate| {
        cells
            .get()
            .get(coord)
            .map(|coord| coord.to_owned())
            .unwrap_or_else(|| Cell {
                coord: coord.clone(),
                ..Default::default()
            })
    };

    let update_cell_text = move |coord: &Coordinate, cell: &Cell, event: &Event| {
        set_cells.update(|cells| {
            cells.insert(
                coord.clone(),
                Cell {
                    text: event_target_value(event),
                    ..cell.clone()
                },
            );
        });
    };

    create_effect(cx, move |_| {
        log!("{:?}", cells.get());
    });

    view! { cx,
        <div class="m-0 relative">
            <For
                each=move || coords.get()
                key=|coord| coord.clone()
                view=move |cx, coord: Coordinate| {
                    let cell = get_cell(&coord);
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
                                prop:value=&cell.text
                                on:input=move |event| update_cell_text(&coord, &cell, &event)
                            />
                        </div>
                }
                }
            />
        </div>
    }
}
