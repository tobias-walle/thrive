use std::str::FromStr;

use regex::Regex;
use serde::{Deserialize, Serialize};
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

#[cfg(test)]
mod tests {
    use super::*;

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
