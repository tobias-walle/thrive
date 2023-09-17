use shared::Coordinate;

use crate::models::{FormatPixel, SignalPair, TableDimensions};

use crate::prelude::*;

#[component]
pub fn RowLabels(cx: Scope, n_rows: i64) -> impl IntoView {
    let (dimensions, _) = use_context::<SignalPair<TableDimensions>>(cx).expect("Missing context");
    view! {
        cx,
        <div class="table-layout-row-labels sticky z-10 left-0 top-0">
            <div
                class="sticky top-0 z-11 bg-slate-200 border-black border-b border-r"
                style:height=move || dimensions.get().row_height.px()
                style:width=move || dimensions.get().labels_width.px()
            />
            <For
                each=move || 0..n_rows
                key=|i| *i
                view=move |cx, row| {
                    view! {
                        cx,
                        <div
                            class="flex items-center justify-center p-1 bg-slate-200 border-black border-b border-r"
                            style:height=move || dimensions.get().row_height.px()
                            style:width=move || dimensions.get().labels_width.px()
                        >{Coordinate::format_row(row)}</div>
                    }
                }
            />
        </div>
    }
}
