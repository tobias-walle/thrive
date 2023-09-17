use crate::prelude::*;

#[component]
pub fn CodeBar(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="table-layout-code-bar flex p-2 w-[100vw] bg-slate-200 border-b border-black">
            <div class="flex-initial p-1 pr-2">𝑓</div>
            <textarea
                rows=1
                class="flex-1 p-1 min-h-[calc(1rem+8px)] font-mono focus:outline-none"
            />
        </div>
    }
}
