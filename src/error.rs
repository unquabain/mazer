use thiserror::*;
use serde::{Serialize,Deserialize};

#[derive(Default,Clone,Debug,Error,Serialize,Deserialize)]
pub enum Error {
    #[error("an error has occurred")]
    #[default]
    Error,

    #[error("an error has occurred: {0}")]
    MessageError(&'static str),

    #[error("edge out of bounds error: index: {0} out of {1}")]
    EdgeOutOfBounds(usize, usize),

    #[error("node out of bounds error: index: {0} out of {1}")]
    NodeOutOfBounds(usize, usize),
}

pub type Result<T> = std::result::Result<T, Error>;
