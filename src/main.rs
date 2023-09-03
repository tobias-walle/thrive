mod app;
mod components;
mod tauri;

use app::*;
use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <App/>
        }
    })
}
