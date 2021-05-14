use crate::sheet::CellId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Command {
    ChangeCellText { id: CellId, text: String },
}
