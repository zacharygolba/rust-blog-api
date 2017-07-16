use std::convert::TryFrom;

use query::error::InvalidValueError;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Fields {
    Authors(Vec<AuthorFields>),
}

impl<'f> TryFrom<(&'f str, Vec<&'f str>)> for Fields {
    type Error = InvalidValueError;

    fn try_from(entry: (&'f str, Vec<&'f str>)) -> Result<Fields, InvalidValueError> {
        let (key, values) = entry;

        match key {
            "authors" => Ok(Fields::Authors(parse_values::<AuthorFields>(values)?)),
            k @ _ => Err(InvalidValueError::fields(
                k.to_owned(),
                String::new(),
                vec![],
            )),
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

impl<'f> TryFrom<&'f str> for AuthorFields {
    type Error = InvalidValueError;

    fn try_from(val: &'f str) -> Result<AuthorFields, InvalidValueError> {
        match val {
            "email" => Ok(AuthorFields::Email),
            "first-name" => Ok(AuthorFields::FirstName),
            "last-name" => Ok(AuthorFields::LastName),
            "created-at" => Ok(AuthorFields::CreatedAt),
            "updated-at" => Ok(AuthorFields::UpdatedAt),
            _ => Err(InvalidValueError::fields(
                String::new(),
                val.to_owned(),
                vec![],
            )),
        }
    }
}

fn parse_values<'f, T>(values: Vec<&'f str>) -> Result<Vec<T>, InvalidValueError>
where
    T: TryFrom<&'f str, Error = InvalidValueError> + 'f,
{
    let mut items = Vec::with_capacity(values.len());

    for field in values.into_iter() {
        items.push(T::try_from(field)?);
    }

    Ok(items)
}
