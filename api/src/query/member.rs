use std::convert::TryFrom;

use rocket::request::{FromForm, FormItems};

use super::decode;
use super::fields::{KEY_REGEX, Entry, Fields};
use super::include::Include;

#[derive(Debug)]
pub struct MemberQuery<F, I>
where
    F: TryFrom<Entry>,
    I: TryFrom<String>,
{
    fields: Option<Fields<F>>,
    include: Option<Include<I>>,
}

impl<F, I> MemberQuery<F, I>
where
    F: TryFrom<(String, Vec<String>)>,
    I: TryFrom<String>,
{
    pub fn new() -> MemberQuery<F, I> {
        MemberQuery::default()
    }

    pub fn fields(&self) -> Option<&Fields<F>> {
        self.fields.as_ref()
    }

    pub fn include(&self) -> Option<&Include<I>> {
        self.include.as_ref()
    }
}

impl<F, I> Default for MemberQuery<F, I>
where
    F: TryFrom<(String, Vec<String>)>,
    I: TryFrom<String>,
{
    fn default() -> MemberQuery<F, I> {
        MemberQuery {
            fields: None,
            include: None,
        }
    }
}

impl<'f, F, I> FromForm<'f> for MemberQuery<F, I>
where
    F: TryFrom<(String, Vec<String>)>,
    I: TryFrom<String>,
{
    type Error = ();

    fn from_form(params: &mut FormItems<'f>, strict: bool) -> Result<MemberQuery<F, I>, ()> {
        let mut query = MemberQuery::new();

        params.filter_map(decode::entry).for_each(
            |(key, value)| match key.as_str() {
                "include" => {
                    query.include = Include::try_from(value).ok();
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
            },
        );

        Ok(query)
    }
}
