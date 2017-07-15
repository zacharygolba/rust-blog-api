use std::io::Read;
use std::ops::{Deref, DerefMut};

use rocket::{Request, Data, Outcome};
use rocket::data::{self, FromData};
use rocket::http::{Status, ContentType};
use rocket::Outcome::*;
use serde::ser::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{self, Map, Value};

use super::{Resource, IntoResource};

const MAX_SIZE: u64 = 1_048_576;

#[derive(Debug, Serialize)]
pub struct Document<D: DeserializeOwned + Serialize> {
    data: D,
}

impl<D: DeserializeOwned + Serialize> Document<D> {
    fn new(data: D) -> Document<D> {
        Document { data }
    }
}

impl<D: DeserializeOwned + Serialize> Deref for Document<D> {
    type Target = D;

    fn deref(&self) -> &D {
        &self.data
    }
}

impl<D: DeserializeOwned + Serialize> DerefMut for Document<D> {
    fn deref_mut(&mut self) -> &mut D {
        &mut self.data
    }
}

impl<T: Default + IntoResource> From<Resource<T>> for Document<Resource<T>> {
    fn from(val: Resource<T>) -> Document<Resource<T>> {
        Document::new(val)
    }
}

impl<T: Default + IntoResource> From<Vec<Resource<T>>> for Document<Vec<Resource<T>>> {
    fn from(val: Vec<Resource<T>>) -> Document<Vec<Resource<T>>> {
        Document::new(val)
    }
}

impl<D: DeserializeOwned + Serialize> FromData for Document<D> {
    type Error = String;

    fn from_data(request: &Request, data: Data) -> data::Outcome<Document<D>, String> {
        if !request.content_type().map_or(false, |mime| mime.is_json()) {
            return Outcome::Forward(data);
        }

        let reader = data.open().take(MAX_SIZE);

        match serde_json::from_reader::<_, D>(reader).map(Document::new) {
            Ok(doc) => Outcome::Success(doc),
            Err(_) => Outcome::Failure((Status::BadRequest, String::from("failed"))),
        }
    }
}
