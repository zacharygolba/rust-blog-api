use std::convert::TryFrom;

use super::csv::ToCsv;

#[derive(Debug)]
pub struct Include<T>
where
    T: TryFrom<String>,
{
    value: Vec<T>,
}

impl<T> Include<T>
where
    T: TryFrom<String>,
{
    pub fn new() -> Include<T> {
        Include::default()
    }

    pub fn try_push(&mut self, value: String) -> Result<(), ()> {
        self.value.push(T::try_from(value).map_err(|_| ())?);
        Ok(())
    }
}

impl<T> Default for Include<T>
where
    T: TryFrom<String>,
{
    fn default() -> Include<T> {
        Include { value: Vec::new() }
    }
}

impl<T, U> TryFrom<T> for Include<U>
where
    T: AsRef<str>,
    U: TryFrom<String>,
{
    type Error = ();

    fn try_from(value: T) -> Result<Include<U>, ()> {
        let mut include = Include::default();

        value.as_ref()
             .to_csv()
             .map(String::from)
             .map(|item| include.try_push(item))
             .find(|res| res.is_err())
             .unwrap_or(Ok(()))?;

        Ok(include)
    }
}
