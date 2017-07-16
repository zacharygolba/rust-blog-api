use std::convert::TryFrom;

use rocket::request::{FromForm, FormItems};

use super::error::InvalidValueError;
use super::fields::{KEY_REGEX, Fields, FieldSet};
use super::include::Include;

#[derive(Debug)]
pub struct MemberParams<'f, F, I>
where
    F: TryFrom<FieldSet<'f>, Error = InvalidValueError> + 'f,
    I: TryFrom<&'f str, Error = InvalidValueError> + 'f,
{
    fields: Fields<'f, F>,
    include: Include<'f, I>,
}

impl<'f, F, I> MemberParams<'f, F, I>
where
    F: TryFrom<FieldSet<'f>, Error = InvalidValueError> + 'f,
    I: TryFrom<&'f str, Error = InvalidValueError> + 'f,
{
    pub fn new() -> MemberParams<'f, F, I> {
        MemberParams::default()
    }

    #[allow(dead_code)]
    pub fn fields(&self) -> &Fields<'f, F> {
        &self.fields
    }

    #[allow(dead_code)]
    pub fn include(&self) -> &Include<'f, I> {
        &self.include
    }
}

impl<'f, F, I> Default for MemberParams<'f, F, I>
where
    F: TryFrom<FieldSet<'f>, Error = InvalidValueError> + 'f,
    I: TryFrom<&'f str, Error = InvalidValueError> + 'f,
{
    fn default() -> MemberParams<'f, F, I> {
        MemberParams {
            fields: Fields::default(),
            include: Include::default(),
        }
    }
}

impl<'f, F, I> FromForm<'f> for MemberParams<'f, F, I>
where
    F: TryFrom<FieldSet<'f>, Error = InvalidValueError> + 'f,
    I: TryFrom<&'f str, Error = InvalidValueError> + 'f,
{
    type Error = InvalidValueError;

    #[allow(unused_variables)]
    fn from_form(
        params: &mut FormItems<'f>,
        strict: bool,
    ) -> Result<MemberParams<'f, F, I>, InvalidValueError> {
        let mut query = MemberParams::new();

        for (key, value) in params {
            match key.as_str() {
                "include" => {
                    query.include = Include::try_from(value)?;
                }
                key @ _ if KEY_REGEX.is_match(key) => {
                    query.fields.try_insert(key, value)?;
                }
                _ => (),
            }
        }

        Ok(query)
    }
}
