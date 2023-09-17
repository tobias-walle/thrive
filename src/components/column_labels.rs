use shared::Coordinate;

use crate::models::{FormatPixel, SignalPair, TableDimensions};

use crate::prelude::*;

#[component]
pub fn ColumnLabels(cx: Scope, n_cols: i64) -> impl IntoView {
    let (dimensions, _) = use_context::<SignalPair<TableDimensions>>(cx).expect("Missing context");
    view! {
        cx,
        <div
            class="table-layout-column-labels sticky z-10 top-0 flex"
            style:left=move || dimensions.get().column_width.px()
        >
            <For
                each=move || 0..n_cols
                key=|i| *i
                view=move |cx, col| {
                    view! {
                        cx,
                        <div
                            class="flex items-center justify-center p-1 bg-slate-200 border-black border-b border-r"
                            style:height=move || dimensions.get().row_height.px()
                            style:width=move || dimensions.get().column_width.px()
                        >{Coordinate::format_column(col)}</div>
                    }
                }
            />
        </div>
    }
}
