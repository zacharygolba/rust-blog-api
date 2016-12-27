use chrono::{UTC, DateTime};

use schema::authors;

#[derive(Debug, Identifiable, Queryable, Serialize)]
pub struct Author {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: DateTime<UTC>,
    pub updated_at: DateTime<UTC>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="authors"]
pub struct NewAuthor {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(AsChangeset, Debug, Deserialize)]
#[table_name="authors"]
pub struct AuthorChanges {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub updated_at: Option<DateTime<UTC>>,
}
