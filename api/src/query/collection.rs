use std::convert::TryFrom;

use rocket::request::{FromForm, FormItems};

use super::decode;
use super::fields::{KEY_REGEX, Entry, Fields};
use super::include::Include;
use super::page::Page;

#[derive(Debug)]
pub struct CollectionQuery<F, I>
where
    F: TryFrom<Entry>,
    I: TryFrom<String>,
{
    fields: Option<Fields<F>>,
    include: Option<Include<I>>,
    page: Option<Page>,
}

impl<F, I> CollectionQuery<F, I>
where
    F: TryFrom<(String, Vec<String>)>,
    I: TryFrom<String>,
{
    pub fn new() -> CollectionQuery<F, I> {
        CollectionQuery::default()
    }

    pub fn fields(&self) -> Option<&Fields<F>> {
        self.fields.as_ref()
    }

    pub fn include(&self) -> Option<&Include<I>> {
        self.include.as_ref()
    }

    pub fn page(&self) -> Option<Page> {
        self.page
    }
}

impl<F, I> Default for CollectionQuery<F, I>
where
    F: TryFrom<(String, Vec<String>)>,
    I: TryFrom<String>,
{
    fn default() -> CollectionQuery<F, I> {
        CollectionQuery {
            fields: None,
            include: None,
            page: None,
        }
    }
}

impl<'f, F, I> FromForm<'f> for CollectionQuery<F, I>
where
    F: TryFrom<(String, Vec<String>)>,
    I: TryFrom<String>,
{
    type Error = ();

    fn from_form(params: &mut FormItems<'f>, strict: bool) -> Result<CollectionQuery<F, I>, ()> {
        let mut query = CollectionQuery::new();

        params.filter_map(decode::entry).for_each(|(key, value)| {
            println!("key({}) => val({})", key, value);
            println!("{:?} - IS FIELD?: {}", *KEY_REGEX, KEY_REGEX.is_match(&key));

            match key.as_str() {
                "include" => {
                    query.include = Include::try_from(value).ok();
                }
                "page[number]" => {
                    let size = match query.page {
                        Some(ref page) => page.size(),
                        _ => Page::DEFAULT_SIZE,
                    };

                    match value.parse::<u64>() {
                        Ok(num) => query.page = Some(Page::new(num, size)),
                        Err(_) => unimplemented!(),
                    }
                }
                "page[size]" => {
                    let num = match query.page {
                        Some(ref page) => page.number(),
                        _ => 1,
                    };

                    match value.parse::<u64>() {
                        Ok(size) => query.page = Some(Page::new(num, size)),
                        Err(_) => unimplemented!(),
                    }
                }
                key @ _ if KEY_REGEX.is_match(key) => {
                    match query.fields {
                        Some(ref mut fields) => {
                            fields.try_insert(key, value);
                        }
                        None => {
                            let mut fields = Fields::default();

                            fields.try_insert(key, value);
                            query.fields = Some(fields);
                        }
                    }
                }
                _ => (),
            }
        });

        Ok(query)
    }
}
