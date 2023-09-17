#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::match_bool)]
#![allow(confusable_idents)]
#![allow(uncommon_codepoints)]

mod app;
mod components;
mod macros;
mod models;
mod prelude;
mod tauri;

use app::App;
use leptos::{mount_to_body, view, warn};

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    mount_to_body(|cx| {
        view! { cx,
            <App/>
        }
    });
}
