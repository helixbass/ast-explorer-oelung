use smol_str::SmolStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("parse error: {0}")]
    Parse(SmolStr),
    #[error("washtank error: {0}")]
    Washtank(SmolStr),
}

pub type Result<TSuccess> = std::result::Result<TSuccess, Error>;
