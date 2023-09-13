use crate::models::{FormatPixel, TableDimensions};

use crate::prelude::*;

#[component]
pub fn ColumnLabels(
    cx: Scope,
    n_cols: i64,
    dimensions: RwSignal<TableDimensions>,
) -> impl IntoView {
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
                            class="
                            flex items-center justify-center
                            p-1
                            bg-slate-200
                            border-black border-b border-r
                            "
                            style:height=move || dimensions.get().row_height.px()
                            style:width=move || dimensions.get().column_width.px()
                        >{col + 1}</div>
                    }
                }
            />
        </div>
    }
}
