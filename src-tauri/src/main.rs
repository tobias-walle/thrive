// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(clippy::pedantic)]
#![allow(clippy::match_bool)]
use pyo3::Python;
use shared::TableCell;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! You've been greeted from Rust!")
}

#[tauri::command]
fn compute(cell: TableCell) -> TableCell {
    let computed = match cell.get_formula() {
        Some(formula) => {
            pyo3::prepare_freethreaded_python();
            Python::with_gil(|py| match py.eval(formula, None, None) {
                Ok(result) => format!("{result}"),
                Err(e) => format!("{e}"),
            })
        }
        None => cell.text.clone(),
    };
    TableCell { computed, ..cell }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, compute])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
