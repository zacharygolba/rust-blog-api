use chrono::{DateTime, Utc};

use model::Model;
use schema::tags;

#[derive(Debug, Identifiable, Queryable, Serialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Model for Tag {}
