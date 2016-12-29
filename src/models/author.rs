use chrono::{UTC, DateTime};

use super::schema::{authors, posts};

#[has_many(posts)]
#[table_name = "authors"]
#[derive(Associations, Debug, Identifiable, Queryable, Serialize)]
pub struct Author {
    pub id: i64,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: DateTime<UTC>,
    pub updated_at: DateTime<UTC>,
}

#[table_name="authors"]
#[derive(Debug, Deserialize, Insertable)]
pub struct NewAuthor {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[table_name="authors"]
#[derive(AsChangeset, Debug, Deserialize)]
pub struct AuthorChangeSet {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub updated_at: Option<DateTime<UTC>>,
}
