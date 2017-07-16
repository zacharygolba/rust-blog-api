use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct InvalidValueError {
    kind: InvalidValue,
    value: String,
    expected: Vec<String>,
}

impl InvalidValueError {
    pub fn fields(key: String, value: String, expected: Vec<String>) -> InvalidValueError {
        InvalidValueError {
            kind: InvalidValue::Fields(key),
            value,
            expected,
        }
    }

    pub fn include(value: String, expected: Vec<String>) -> InvalidValueError {
        InvalidValueError {
            kind: InvalidValue::Include,
            value,
            expected,
        }
    }
}

impl Display for InvalidValueError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let path = match self.kind {
            InvalidValue::Fields(ref key) => String::from("fields.") + key,
            InvalidValue::Include => String::from("include"),
        };

        write!(
            f,
            "Expected value for parameter \"{}\" to be one of [{}] but got {}",
            path,
            self.expected.join(", "),
            self.value,
        )
    }
}

impl Error for InvalidValueError {
    fn cause(&self) -> Option<&Error> {
        Some(self)
    }

    fn description(&self) -> &str {
        "An invalid query parameter was received."
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum InvalidValue {
    Fields(String),
    Include,
}
