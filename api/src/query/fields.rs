use std::convert::TryFrom;

use regex::Regex;

lazy_static! {
    pub static ref KEY_REGEX: Regex = Regex::new(r"fields\[(.+)\]").expect("Invalid regexp.");
}

pub type Entry = (String, Vec<String>);

#[derive(Debug)]
pub struct Fields<T>
where
    T: TryFrom<Entry>,
{
    value: Vec<T>,
}

impl<T> Fields<T>
where
    T: TryFrom<Entry>,
{
    pub fn new() -> Fields<T> {
        Fields::default()
    }

    pub fn try_insert<K, V>(&mut self, key: K, value: V) -> Result<(), ()>
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let entry = (extract_key(key), extract_value(value));

        self.value.push(T::try_from(entry).map_err(|_| ())?);
        Ok(())
    }
}

impl<T> Default for Fields<T>
where
    T: TryFrom<Entry>,
{
    fn default() -> Fields<T> {
        Fields { value: Vec::new() }
    }
}

fn extract_key<T>(key: T) -> String
where
    T: AsRef<str>,
{
    KEY_REGEX.captures(key.as_ref())
             .unwrap()
             .get(1)
             .unwrap()
             .as_str()
             .into()
}

fn extract_value<T>(value: T) -> Vec<String>
where
    T: AsRef<str>,
{
    value.as_ref()
         .split(',')
         .map(|item| item.trim())
         .map(String::from)
         .collect()
}
