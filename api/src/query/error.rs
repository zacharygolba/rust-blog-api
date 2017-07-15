use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum QueryError {
    InvalidQueryParam(String),
}

impl Display for QueryError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            QueryError::InvalidQueryParam(ref err) => write!(f, "{}", err),
        }
    }
}

impl Error for QueryError {
    fn cause(&self) -> Option<&Error> {
        Some(self)
    }

    fn description(&self) -> &str {
        match *self {
            QueryError::InvalidQueryParam(ref param) => param,
        }
    }
}
