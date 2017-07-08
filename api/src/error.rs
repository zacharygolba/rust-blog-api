use rocket_contrib::JSON;
use rocket::response::Failure;

pub type Result<T> = ::std::result::Result<T, Failure>;
pub type JSONResult<T> = Result<JSON<T>>;
