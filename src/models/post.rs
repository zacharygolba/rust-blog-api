use chrono::{UTC, DateTime};

use super::author::Author;
use super::schema::posts;

#[belongs_to(Author)]
#[table_name = "posts"]
#[derive(Associations, Debug, Identifiable, Queryable, Serialize)]
pub struct Post {
    pub id: i64,
    pub body: String,
    pub title: String,
    pub published: bool,
    pub created_at: DateTime<UTC>,
    pub updated_at: DateTime<UTC>,
    pub author_id: i64,
}

#[table_name="posts"]
#[derive(Debug, Deserialize, Insertable)]
pub struct NewPost {
    pub body: String,
    pub title: String,
    pub author_id: i64,
}

#[table_name="posts"]
#[derive(AsChangeset, Debug, Deserialize)]
pub struct PostChangeSet {
    pub body: Option<String>,
    pub title: Option<String>,
    pub published: Option<bool>,
    pub updated_at: Option<DateTime<UTC>>,
}
