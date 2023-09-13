use shared::{Coordinate, TableState};

use crate::components::{Border, BorderDirection, Cell};
use crate::models::TableDimensions;
use crate::prelude::*;

#[component]
pub fn Table(cx: Scope) -> impl IntoView {
    let n_rows = 10;
    let n_cols = 10;
    let (coords, _set_coords) = create_signal(
        cx,
        (0..n_rows)
            .flat_map(|row| (0..n_cols).map(move |col| Coordinate { col, row }))
            .collect::<Vec<_>>(),
    );
    let state = create_rw_signal(cx, TableState::new());
    let dimensions = create_rw_signal(cx, TableDimensions::new());

    create_effect(cx, move |_| {
        log!("{:#?}", state.get());
    });

    view! { cx,
        <div class="m-0 relative">
            <For
                each=move || coords.get()
                key=|coord| *coord
                view=move |cx, coord: Coordinate| view! {
                    cx,
                    <Cell
                        coord=coord
                        state=state
                        dimensions=dimensions
                    />
                    <Border
                        coord=coord
                        direction=BorderDirection::Top
                        dimensions=dimensions
                    />
                    <Border
                        coord=coord
                        direction=BorderDirection::Left
                        dimensions=dimensions
                    />
                    // Render one additional horizontal border in last row
                    <Show
                        when=move || coord.row == n_rows - 1
                        fallback=move |_| ()
                    >
                        <Border
                            coord=Coordinate::new(coord.col, coord.row + 1)
                            direction=BorderDirection::Top
                            dimensions=dimensions
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
                            dimensions=dimensions
                        />
                    </Show>
                }
            />
        </div>
    }
}
