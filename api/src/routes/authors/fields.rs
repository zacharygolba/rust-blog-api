use std::convert::TryFrom;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Fields {
    Authors(Vec<AuthorFields>),
}

impl TryFrom<(String, Vec<String>)> for Fields {
    type Error = ();

    fn try_from(entry: (String, Vec<String>)) -> Result<Fields, ()> {
        let items = entry.1
                         .into_iter()
                         .filter_map(|field| AuthorFields::try_from(field).ok())
                         .collect();

        match entry.0.as_str() {
            "authors" => Ok(Fields::Authors(items)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum AuthorFields {
    Email,
    FirstName,
    LastName,
    CreatedAt,
    UpdatedAt,
}

impl TryFrom<String> for AuthorFields {
    type Error = ();

    fn try_from(val: String) -> Result<AuthorFields, ()> {
        match val.as_str() {
            "email" => Ok(AuthorFields::Email),
            "first-name" => Ok(AuthorFields::FirstName),
            "last-name" => Ok(AuthorFields::LastName),
            "created-at" => Ok(AuthorFields::CreatedAt),
            "updated-at" => Ok(AuthorFields::UpdatedAt),
            _ => Err(()),
        }
    }
}
