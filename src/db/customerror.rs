use std::fmt;
use tokio_postgres::Error;

#[cfg(feature = "sql-server")]
use std::io::Error as IOError;
#[cfg(feature = "sql-server")]
use tiberius::error::Error as SQLError;

#[derive(Debug)]
pub enum CustomError {
    EmptyVec,
    PGError(tokio_postgres::Error),
    #[cfg(feature = "sql-server")]
    SQLServerError(SQLError),
    #[cfg(feature = "sql-server")]
    IOError(IOError),
}

impl From<tokio_postgres::Error> for CustomError {
    fn from(err: Error) -> CustomError {
        CustomError::PGError(err)
    }
}

#[cfg(feature = "sql-server")]
impl From<IOError> for CustomError {
    fn from(err: IOError) -> CustomError {
        CustomError::IOError(err)
    }
}

#[cfg(feature = "sql-server")]
impl From<SQLError> for CustomError {
    fn from(err: SQLError) -> CustomError {
        CustomError::SQLServerError(err)
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::EmptyVec => write!(f, "No rows returned"),
            CustomError::PGError(ref err) => err.fmt(f),
            #[cfg(feature = "sql-server")]
            CustomError::SQLServerError(ref err) => err.fmt(f),
            #[cfg(feature = "sql-server")]
            CustomError::IOError(ref err) => err.fmt(f),
        }
    }
}
