use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableDimensions {
    pub column_width: i64,
    pub row_height: i64,
    pub border_width: i64,
    pub labels_width: i64,
}

impl TableDimensions {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for TableDimensions {
    fn default() -> Self {
        Self {
            column_width: 100,
            row_height: 24,
            border_width: 1,
            labels_width: 30,
        }
    }
}

pub trait FormatPixel {
    fn px(&self) -> String;
}

impl FormatPixel for i64 {
    fn px(&self) -> String {
        format!("{self}px")
    }
}
