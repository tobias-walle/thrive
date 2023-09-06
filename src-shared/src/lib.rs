use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, Hash, PartialEq, Eq)]
pub struct Coordinate {
    pub row: i64,
    pub col: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TableCell {
    pub coord: Coordinate,
    pub text: String,
    pub computed: String,
}

impl TableCell {
    pub fn has_formula(&self) -> bool {
        self.text.starts_with('=')
    }

    pub fn get_formula(&self) -> Option<&str> {
        if self.has_formula() {
            Some(&self.text[1..])
        } else {
            None
        }
    }
}
