use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::Serializer;
use serde_with::serde_as;
use shared::{Coordinate, TableCellWithCoordinates, TableState};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[serde_as]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ComputeArgs {
    state: TableState,
    coord: Coordinate,
}

pub async fn compute(state: TableState, coord: Coordinate) -> Vec<TableCellWithCoordinates> {
    let serializer = Serializer::json_compatible();
    let args = ComputeArgs { state, coord }
        .serialize(&serializer)
        .expect("Failed to serialize args");
    let result = invoke("compute", args).await;
    serde_wasm_bindgen::from_value(result).expect("Failed to convert result to struct")
}
