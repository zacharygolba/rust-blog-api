use blog_fmt::jsonapi::{Attributes, IntoResource};
use chrono::{DateTime, Utc};
use serde_json::Value;

pub use schema::authors;

#[derive(Debug, Identifiable, Queryable, Serialize)]
#[serde(rename_all = "kebab-case")]
#[table_name = "authors"]
pub struct Author {
    pub id: i64,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Deserialize, Insertable, Serialize)]
#[table_name = "authors"]
#[serde(rename_all = "kebab-case")]
pub struct NewAuthor {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}
