use std::convert::TryFrom;

use rocket::request::{FromForm, FormItems};

use super::error::InvalidValueError;
use super::fields::{KEY_REGEX, Fields, FieldSet};
use super::include::Include;
use super::page::Page;

#[derive(Debug)]
pub struct CollectionParams<'f, F, I>
where
    F: TryFrom<FieldSet<'f>, Error = InvalidValueError> + 'f,
    I: TryFrom<&'f str, Error = InvalidValueError> + 'f,
{
    fields: Fields<'f, F>,
    include: Include<'f, I>,
    page: Page,
}

impl<'f, F, I> CollectionParams<'f, F, I>
where
    F: TryFrom<FieldSet<'f>, Error = InvalidValueError> + 'f,
    I: TryFrom<&'f str, Error = InvalidValueError> + 'f,
{
    pub fn new() -> CollectionParams<'f, F, I> {
        CollectionParams::default()
    }

    #[allow(dead_code)]
    pub fn fields(&self) -> &Fields<'f, F> {
        &self.fields
    }

    #[allow(dead_code)]
    pub fn include(&self) -> &Include<'f, I> {
        &self.include
    }

    pub fn page(&self) -> Page {
        self.page
    }
}

impl<'f, F, I> Default for CollectionParams<'f, F, I>
where
    F: TryFrom<FieldSet<'f>, Error = InvalidValueError> + 'f,
    I: TryFrom<&'f str, Error = InvalidValueError> + 'f,
{
    fn default() -> CollectionParams<'f, F, I> {
        CollectionParams {
            fields: Fields::default(),
            include: Include::default(),
            page: Page::default(),
        }
    }
}

impl<'f, F, I> FromForm<'f> for CollectionParams<'f, F, I>
where
    F: TryFrom<FieldSet<'f>, Error = InvalidValueError> + 'f,
    I: TryFrom<&'f str, Error = InvalidValueError> + 'f,
{
    type Error = InvalidValueError;

    #[allow(unused_variables)]
    fn from_form(
        params: &mut FormItems<'f>,
        strict: bool,
    ) -> Result<CollectionParams<'f, F, I>, InvalidValueError> {
        let mut query = CollectionParams::new();

        for (key, value) in params {
            match key.as_str() {
                "include" => {
                    query.include = Include::try_from(value)?;
                }
                "page%5Bnumber%5D" => {
                    let size = query.page().size();

                    match value.parse::<u64>() {
                        Ok(num) => query.page = Page::new(num, size),
                        Err(_) => unimplemented!(),
                    }
                }
                "page%5Bsize%5D" => {
                    let num = query.page().number();

                    match value.parse::<u64>() {
                        Ok(size) => query.page = Page::new(num, size),
                        Err(_) => unimplemented!(),
                    }
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
