use std::convert::TryFrom;
use std::marker::PhantomData;

use regex::Regex;

use super::error::InvalidValueError;

lazy_static! {
    pub static ref KEY_REGEX: Regex = Regex::new(r"fields%5B(.+)%5D").expect("Invalid regexp.");
}

pub type FieldSet<'f> = (&'f str, Vec<&'f str>);

#[derive(Debug)]
pub struct Fields<'f, T>
where
    T: TryFrom<FieldSet<'f>, Error = InvalidValueError> + 'f,
{
    value: Vec<T>,
    form: PhantomData<&'f T>,
}

impl<'f, T> Fields<'f, T>
where
    T: TryFrom<FieldSet<'f>, Error = InvalidValueError>,
{
    pub fn try_insert(&mut self, key: &'f str, value: &'f str) -> Result<(), InvalidValueError> {
        self.value
            .push(T::try_from((extract_key(key), extract_value(value)))?);
        Ok(())
    }
}

impl<'f, T> Default for Fields<'f, T>
where
    T: TryFrom<FieldSet<'f>, Error = InvalidValueError>,
{
    fn default() -> Fields<'f, T> {
        Fields {
            value: Vec::new(),
            form: PhantomData,
        }
    }
}

fn extract_key<'f>(key: &'f str) -> &'f str {
    KEY_REGEX.captures(key).unwrap().get(1).unwrap().as_str()
}

fn extract_value<'f>(value: &'f str) -> Vec<&'f str> {
    value.split(',').map(|item| item.trim()).collect()
}
