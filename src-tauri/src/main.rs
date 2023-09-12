#![warn(clippy::pedantic)]
#![allow(clippy::match_bool)]
#![allow(clippy::similar_names)]
#![allow(clippy::needless_update)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod compute;

use shared::{Coordinate, TableCellWithCoordinates, TableState};

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
fn compute(state: TableState, coord: Coordinate) -> Vec<TableCellWithCoordinates> {
    compute::compute(state, coord)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![compute])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
