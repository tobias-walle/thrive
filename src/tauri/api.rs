use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use shared::TableCell;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct ComputeArgs {
    cell: TableCell,
}

pub async fn compute(cell: TableCell) -> TableCell {
    let args = to_value(&ComputeArgs { cell }).unwrap();
    let result = invoke("compute", args).await;
    serde_wasm_bindgen::from_value(result).unwrap()
}
