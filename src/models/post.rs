use chrono::{UTC, DateTime};

use schema::posts;

#[derive(Debug, Identifiable, Queryable, Serialize)]
pub struct Post {
    pub id: i64,
    pub body: String,
    pub title: String,
    pub published: bool,
    pub created_at: DateTime<UTC>,
    pub updated_at: DateTime<UTC>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

#[derive(AsChangeset, Debug, Deserialize)]
#[table_name="posts"]
pub struct PostChanges {
    pub body: Option<String>,
    pub title: Option<String>,
    pub published: Option<bool>,
    pub updated_at: Option<DateTime<UTC>>,
}
