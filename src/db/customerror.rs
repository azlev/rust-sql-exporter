use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    EmptyVec,
    DBError(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::EmptyVec => write!(f, "No rows returned"),
            CustomError::DBError(ref err) => err.fmt(f),
        }
    }
}
