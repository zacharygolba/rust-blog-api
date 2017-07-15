use std::convert::TryFrom;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Include {
    Posts,
}

impl TryFrom<String> for Include {
    type Error = ();

    fn try_from(value: String) -> Result<Include, ()> {
        match value.as_str() {
            "posts" => Ok(Include::Posts),
            _ => Err(()),
        }
    }
}
