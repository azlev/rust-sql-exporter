use std::fmt;
use tokio_postgres::Error;

#[derive(Debug)]
pub enum CustomError {
    EmptyVec,
    PGError(tokio_postgres::Error),
}

impl From<tokio_postgres::Error> for CustomError {
    fn from(err: Error) -> CustomError {
        CustomError::PGError(err)
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::EmptyVec => write!(f, "No rows returned"),
            CustomError::PGError(ref err) => err.fmt(f),
        }
    }
}
