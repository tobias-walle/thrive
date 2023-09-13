use std::{collections::HashMap, sync::OnceLock};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::models::Coordinate;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TableCell {
    pub text: String,
    pub computed: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TableCellWithCoordinates {
    pub coord: Coordinate,
    pub cell: TableCell,
}

impl TableCell {
    pub fn default_ref() -> &'static TableCell {
        static DEFAULT_CELL: OnceLock<TableCell> = OnceLock::new();
        DEFAULT_CELL.get_or_init(TableCell::default)
    }

    #[must_use]
    pub fn with_coord(self, coord: Coordinate) -> TableCellWithCoordinates {
        TableCellWithCoordinates { coord, cell: self }
    }

    #[must_use]
    pub fn has_formula(&self) -> bool {
        self.text.starts_with('=')
    }

    #[must_use]
    pub fn get_formula(&self) -> Option<&str> {
        if self.has_formula() {
            Some(&self.text[1..])
        } else {
            None
        }
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TableState {
    #[serde_as(as = "Vec<(_, _)>")]
    cells: HashMap<Coordinate, TableCell>,
}

impl TableState {
    #[must_use]
    pub fn new() -> Self {
        TableState::default()
    }

    pub fn set_cell(&mut self, coord: Coordinate, cell: TableCell) {
        self.cells.insert(coord, cell);
    }

    #[must_use]
    pub fn cell(&self, coord: &Coordinate) -> &TableCell {
        match self.cells.get(coord) {
            Some(cell) => cell,
            None => TableCell::default_ref(),
        }
    }

    #[must_use]
    pub fn cell_mut(&mut self, coord: &Coordinate) -> Option<&mut TableCell> {
        self.cells.get_mut(coord)
    }
}

impl<const N: usize> From<[TableCellWithCoordinates; N]> for TableState {
    fn from(cells: [TableCellWithCoordinates; N]) -> Self {
        let mut state = TableState::new();
        for cell in cells {
            state.set_cell(cell.coord, cell.cell);
        }
        state
    }
}
