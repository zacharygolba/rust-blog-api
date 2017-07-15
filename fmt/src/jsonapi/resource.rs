use std::ops::{Deref, DerefMut};

use rocket::{Request, Data, Outcome};
use rocket::data::{self, FromData};
use rocket::http::{Status, ContentType};
use rocket::Outcome::*;
use serde_json::Value;

pub trait IntoResource: Sized {
    fn id(&self) -> String;
    fn kind(&self) -> String;
    fn attributes(&self) -> Value;
    fn relationships(&self) -> Option<Value>;

    fn into_resource(self) -> Resource<Self> {
        Resource::from(self)
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Resource<T: IntoResource> {
    pub id: String,

    #[serde(rename = "type")]
    pub kind: String,

    pub attributes: Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Value>,

    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub links: Option<Value>,

    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub meta: Option<Value>,
    #[serde(skip_deserializing, skip_serializing)]
    source: T,
}

impl<T: IntoResource> Deref for Resource<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.source
    }
}

impl<T: IntoResource> DerefMut for Resource<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.source
    }
}

impl<T: IntoResource> From<T> for Resource<T> {
    fn from(source: T) -> Resource<T> {
        let id = source.id();
        let kind = source.kind();
        let attributes = source.attributes();
        let relationships = source.relationships();

        Resource {
            id,
            kind,
            attributes,
            relationships,
            source,
        }
    }
}
