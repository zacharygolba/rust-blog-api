use chrono::{UTC, DateTime};

use super::schema::{authors, posts};

#[has_many(posts)]
#[table_name = "authors"]
#[derive(Associations, Debug, Identifiable, Queryable, Serialize)]
pub struct Author {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: DateTime<UTC>,
    pub updated_at: DateTime<UTC>,
}

#[table_name="authors"]
#[derive(Debug, Deserialize, Insertable)]
pub struct NewAuthor {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[table_name="authors"]
#[derive(AsChangeset, Debug, Deserialize)]
pub struct AuthorChangeSet {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub updated_at: Option<DateTime<UTC>>,
}
