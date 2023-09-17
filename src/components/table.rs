use shared::{Coordinate, TableState};

use crate::components::{Border, BorderDirection, Cell, CodeBar, ColumnLabels, RowLabels};
use crate::models::TableDimensions;
use crate::prelude::*;

#[component]
pub fn Table(cx: Scope) -> impl IntoView {
    let n_rows = 30;
    let n_cols = 10;
    let (coords, _set_coords) = create_signal(
        cx,
        (0..n_rows)
            .flat_map(|row| (0..n_cols).map(move |col| Coordinate { col, row }))
            .collect::<Vec<_>>(),
    );

    let (state, set_state) = create_signal(cx, TableState::new());
    provide_context(cx, (state, set_state));

    let (dimensions, set_dimensions) = create_signal(cx, TableDimensions::new());
    provide_context(cx, (dimensions, set_dimensions));

    create_effect(cx, move |_| {
        log!("{:#?}", state.get());
    });

    view! { cx,
        <div class="table-layout">
            <CodeBar/>
            <ColumnLabels n_cols=n_cols/>
            <RowLabels n_rows=n_rows/>
            <div class="table-layout-table relative flex-1">
                <For
                    each=move || coords.get()
                    key=|coord| *coord
                    view=move |cx, coord: Coordinate| view! {
                        cx,
                        <Cell coord=coord/>
                        <Border coord=coord direction=BorderDirection::Top/>
                        <Border coord=coord direction=BorderDirection::Left/>
                        // Render one additional horizontal border in last row
                        <Show
                            when=move || coord.row == n_rows - 1
                            fallback=move |_| ()
                        >
                            <Border
                                coord=Coordinate::new(coord.col, coord.row + 1)
                                direction=BorderDirection::Top
                            />
                        </Show>
                        // Render one additional vertical border in last column
                        <Show
                            when=move || coord.col == n_cols - 1
                            fallback=move |_| ()
                        >
                            <Border
                                coord=Coordinate::new(coord.col + 1, coord.row)
                                direction=BorderDirection::Left
                            />
                        </Show>
                    }
                />
            </div>
        </div>
    }
}
