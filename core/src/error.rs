use thiserror::Error;

use crate::sheet::CellId;
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("Couldn't find cell with id {0}")]
    CellIdNotFound(CellId),
}

pub type Result<T> = std::result::Result<T, Error>;
