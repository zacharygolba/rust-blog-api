use chrono::{DateTime, Utc};

use super::Model;

#[derive(Debug, Queryable, Serialize)]
pub struct Comment {
    pub id: i64,
    pub edited: bool,
    pub message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Model for Comment {}
