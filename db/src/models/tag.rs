use chrono::{DateTime, Utc};

use super::Model;

#[derive(Debug, Queryable, Serialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Model for Tag {}
