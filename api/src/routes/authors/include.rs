use std::convert::TryFrom;

use query::error::InvalidValueError;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Include {
    Posts,
}

impl<'f> TryFrom<&'f str> for Include {
    type Error = InvalidValueError;

    fn try_from(value: &'f str) -> Result<Include, InvalidValueError> {
        match value {
            "posts" => Ok(Include::Posts),
            _ => Err(InvalidValueError::include(value.to_owned(), vec![])),
        }
    }
}
