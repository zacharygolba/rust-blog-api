use std::{error, result};
use std::fmt::{self, Display, Formatter};

use blog_fmt;
use rocket_contrib::Json;
use rocket::response::Failure;
use rocket::response::status::Created;

pub type CreatedResult<T> = Result<Created<Json<T>>>;
pub type JsonResult<T> = Result<Json<T>>;

pub type Result<T> = result::Result<T, Failure>;

#[derive(Debug)]
pub enum Error {
    Fmt(blog_fmt::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Error::Fmt(ref err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Fmt(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Fmt(ref err) => Some(err),
        }
    }
}

impl From<blog_fmt::Error> for Error {
    fn from(err: blog_fmt::Error) -> Error {
        Error::Fmt(err)
    }
}
