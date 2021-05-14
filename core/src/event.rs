use crate::sheet::CellId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Event {
    CellTextChanged { id: CellId, text: String },
}
