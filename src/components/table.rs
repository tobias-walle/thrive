use shared::{Coordinate, State};

use crate::components::TableCell;
use crate::prelude::*;

#[component]
pub fn Table(cx: Scope) -> impl IntoView {
    let (coords, _set_coords) = create_signal(
        cx,
        (0..10)
            .flat_map(|row| (0..10).map(move |col| Coordinate { col, row }))
            .collect::<Vec<_>>(),
    );
    let (state, set_state) = create_signal(cx, State::new());

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
                    <TableCell
                        coord=coord
                        state=state
                        set_state=set_state
                    />
                }
            />
        </div>
    }
}
