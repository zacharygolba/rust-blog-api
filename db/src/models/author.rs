use chrono::{DateTime, Utc};

use super::Model;

#[derive(Debug, Queryable, Serialize)]
pub struct Author {
    pub created_at: DateTime<Utc>,
    pub email: String,
    pub first_name: String,
    pub id: i64,
    pub last_name: String,
    pub updated_at: DateTime<Utc>,
}

impl Model for Author {}
