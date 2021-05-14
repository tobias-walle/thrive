use derive_more::Display;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::{Error, Result};

#[derive(Debug, Clone, Hash)]
pub struct Position {
    row: i32,
    col: i32,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, Display, Serialize, Deserialize)]
pub struct CellId(#[display] String);

#[derive(Debug, Clone)]
pub struct Cell {
    text: String,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            text: "".to_string(),
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new()
    }
}

impl CellId {
    fn new(id: String) -> Self {
        Self(id)
    }

    fn random() -> Self {
        Self::new(nanoid!())
    }
}

#[derive(Debug, Clone)]
pub struct Sheet {
    cells_by_id: HashMap<CellId, Cell>,
    cell_positions: HashMap<Position, CellId>,
}

impl Sheet {
    pub fn new() -> Self {
        Self {
            cells_by_id: HashMap::new(),
            cell_positions: HashMap::new(),
        }
    }

    pub fn create_cell(&mut self) -> CellId {
        let id = CellId::random();
        let cell = Cell::new();
        self.cells_by_id.insert(id.clone(), cell);
        id
    }

    pub fn get_cell_text(&self, id: &CellId) -> Option<&str> {
        Some(&self.cells_by_id.get(id)?.text)
    }

    pub fn set_cell_text(&mut self, id: &CellId, text: String) -> Result<()> {
        let mut cell = self
            .cells_by_id
            .get_mut(id)
            .ok_or_else(|| Error::CellIdNotFound(id.clone()))?;
        cell.text = text;
        Ok(())
    }
}

impl Default for Sheet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_set_and_get_cell_values() {
        let mut sheet = Sheet::new();

        let c1 = sheet.create_cell();
        let c2 = sheet.create_cell();

        sheet.set_cell_text(&c2, "1 + 1".into()).unwrap();

        assert_eq!(sheet.get_cell_text(&c1), Some(""));
        assert_eq!(sheet.get_cell_text(&c2), Some("1 + 1"));
        assert_eq!(sheet.get_cell_text(&CellId::random()), None);
    }
}
