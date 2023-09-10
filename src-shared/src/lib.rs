#![warn(clippy::pedantic)]

use std::{collections::HashMap, str::FromStr};

use once_cell::sync::OnceCell;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use thiserror::Error;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, Hash, PartialEq, Eq)]
pub struct Coordinate {
    pub col: i64,
    pub row: i64,
}

impl Coordinate {
    #[must_use]
    pub fn new(col: i64, row: i64) -> Self {
        Self { col, row }
    }
}

#[derive(Error, Debug)]
#[error("Failed to parse coordinate: {0}")]
pub struct ParseCoordinateError(String);

impl FromStr for Coordinate {
    type Err = ParseCoordinateError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let error = || ParseCoordinateError(input.into());
        let re = Regex::new(r"^([A-Z]+)(\d+)$").unwrap();
        let capture = re.captures(input).ok_or_else(error)?;

        let mut col = 0;
        let letters = capture[1].to_string();
        let count = letters.chars().count();
        for (i, c) in letters.chars().enumerate() {
            let c = c as i64;
            let factor = 26_i64.pow((count - 1 - i).try_into().map_err(|_| error())?);
            col += (c - 'A' as i64 + 1) * factor;
        }
        col -= 1;

        let row = capture[2].parse::<i64>().map_err(|_| error())? - 1;

        Ok(Self { col, row })
    }
}

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

static DEFAULT_CELL: OnceCell<TableCell> = OnceCell::new();

impl TableCell {
    pub fn default_ref() -> &'static TableCell {
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
pub struct State {
    #[serde_as(as = "Vec<(_, _)>")]
    cells: HashMap<Coordinate, TableCell>,
}

impl State {
    #[must_use]
    pub fn new() -> Self {
        State::default()
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

impl<const N: usize> From<[TableCellWithCoordinates; N]> for State {
    fn from(cells: [TableCellWithCoordinates; N]) -> Self {
        let mut state = State::new();
        for cell in cells {
            state.set_cell(cell.coord, cell.cell);
        }
        state
    }
}

#[cfg(test)]
mod tests {
    use crate::Coordinate;

    #[test]
    fn coordinate_should_be_parsed_from_string() {
        let tests = [
            ("A1", (0, 0)),
            ("B1", (1, 0)),
            ("A2", (0, 1)),
            ("Z1", (25, 0)),
            ("AA1", (26, 0)),
            ("AB1", (27, 0)),
            ("AZ1", (51, 0)),
            ("BA1", (52, 0)),
            ("BC1", (54, 0)),
            ("A10", (0, 9)),
            ("A100", (0, 99)),
            ("BC100", (54, 99)),
            ("ZA1", (676, 0)),
            ("ZZ1", (701, 0)),
            ("AAA1", (702, 0)),
        ];
        for (str, (col, row)) in tests {
            assert_eq!(
                str.parse::<Coordinate>().unwrap(),
                Coordinate::new(col, row),
                "Failed to parse '{str}'"
            );
        }
    }
}
