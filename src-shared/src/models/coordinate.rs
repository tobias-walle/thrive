use std::{fmt::Display, str::FromStr};

use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseCoordinateError {
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
    #[error("Failed to parse coordinate row: {0}")]
    ParseRowFailed(String),
    #[error("Failed to parse coordinate column: {0}")]
    ParseColumnFailed(String),
}

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

    fn parse_row(input: &str) -> Result<i64, ParseCoordinateError> {
        let row = input
            .parse::<i64>()
            .map_err(|_| ParseCoordinateError::ParseRowFailed(input.into()))?;
        Ok(row - 1)
    }

    fn parse_column(input: &str) -> Result<i64, ParseCoordinateError> {
        let mut col = 0;
        let count: u32 = input
            .chars()
            .count()
            .try_into()
            .map_err(|_| ParseCoordinateError::ParseColumnFailed(input.into()))
            .expect("Failed to convert count");
        for (i, c) in input.chars().enumerate() {
            let i: u32 = i
                .try_into()
                .map_err(|_| ParseCoordinateError::ParseColumnFailed(input.into()))?;
            let c = c as i64;
            let factor = 26_i64.pow(count - 1 - i);
            col += (c - 'A' as i64 + 1) * factor;
        }
        Ok(col - 1)
    }

    #[must_use]
    pub fn format_row(row: i64) -> String {
        format!("{}", row + 1)
    }

    #[must_use]
    pub fn format_column(col: i64) -> String {
        let mut output = String::new();
        // Adjust offset because A == 1, not 0
        let mut n = col + 1;
        while n > 0 {
            // adjust to 0 indexing
            n -= 1;
            // Both casts are save, as the we know the range of the numbers.
            #[allow(clippy::cast_sign_loss, clippy::char_lit_as_u8)]
            let c = ((n % 26) as u8 + ('A' as u8)) as char;
            output.push(c);
            n /= 26;
        }
        output.chars().rev().collect()
    }
}

impl FromStr for Coordinate {
    type Err = ParseCoordinateError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^([A-Z]+)(\d+)$").unwrap();
        let capture = re
            .captures(input)
            .ok_or_else(|| ParseCoordinateError::InvalidFormat(input.into()))?;

        Ok(Self {
            col: Coordinate::parse_column(&capture[1])?,
            row: Coordinate::parse_row(&capture[2])?,
        })
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let row = Coordinate::format_row(self.row);
        let col = Coordinate::format_column(self.col);
        write!(f, "{col}{row}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASES: &[(&str, (i64, i64))] = &[
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

    #[test]
    fn coordinate_should_be_parsed_from_string() {
        for (str, (col, row)) in TEST_CASES {
            let coord = Coordinate::new(*col, *row);
            assert_eq!(
                str.parse::<Coordinate>(),
                Ok(coord),
                "Failed to parse '{str}'"
            );
        }
    }

    #[test]
    fn coordinate_should_be_formated_as_string() {
        for (str, (col, row)) in TEST_CASES {
            let coord = Coordinate::new(*col, *row);
            assert_eq!(format!("{coord}"), *str, "Failed to format '{coord:?}'");
        }
    }
}
