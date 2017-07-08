use chrono::{DateTime, Utc};

use super::Model;

#[derive(Debug, Queryable, Serialize)]
pub struct Post {
    pub id: i64,
    // pub author_id: i64,
    pub body: Option<String>,
    pub title: String,
    pub published_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Model for Post {}
