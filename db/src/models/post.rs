use chrono::{DateTime, Utc};

use model::Model;
use schema::posts;

#[derive(Debug, Identifiable, Queryable, Serialize)]
pub struct Post {
    pub id: i64,
    // pub author_id: i64,
    pub title: String,
    pub body: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Model for Post {}
