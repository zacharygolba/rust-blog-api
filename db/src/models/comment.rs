use chrono::{DateTime, Utc};

use model::Model;
use schema::comments;

#[derive(Debug, Identifiable, Queryable, Serialize)]
pub struct Comment {
    pub id: i64,
    pub edited: bool,
    pub message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Model for Comment {}
