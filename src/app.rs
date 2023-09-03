use leptos::*;

use crate::components::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <main>
            <Table/>
        </main>
    }
}
