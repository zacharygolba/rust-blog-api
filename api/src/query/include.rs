use std::convert::TryFrom;
use std::marker::PhantomData;

use super::error::InvalidValueError;

#[derive(Debug)]
pub struct Include<'f, T>
where
    T: TryFrom<&'f str, Error = InvalidValueError> + 'f,
{
    value: Vec<T>,
    form: PhantomData<&'f T>,
}

impl<'f, T> Include<'f, T>
where
    T: TryFrom<&'f str, Error = InvalidValueError> + 'f,
{
    pub fn try_push(&mut self, value: &'f str) -> Result<(), InvalidValueError> {
        self.value.push(T::try_from(value)?);
        Ok(())
    }
}

impl<'f, T> Default for Include<'f, T>
where
    T: TryFrom<&'f str, Error = InvalidValueError> + 'f,
{
    fn default() -> Include<'f, T> {
        Include {
            value: Vec::new(),
            form: PhantomData,
        }
    }
}

impl<'f, T> TryFrom<&'f str> for Include<'f, T>
where
    T: TryFrom<&'f str, Error = InvalidValueError> + 'f,
{
    type Error = InvalidValueError;

    fn try_from(value: &'f str) -> Result<Include<'f, T>, InvalidValueError> {
        let mut include = Include::default();

        value.split(',')
             .map(|item| item.trim())
             .map(|item| include.try_push(item))
             .find(|res| res.is_err())
             .unwrap_or(Ok(()))?;

        Ok(include)
    }
}
