use std::str::FromStr;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::likes;

#[derive(Debug, Deserialize, Serialize)]
pub struct Like {
    pub id: String,
    pub created_at: DateTime<Utc>,
}

impl Like {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
        }
    }

    pub fn to_like_db(&self, tweet_id: Uuid) -> Likes {
        Likes {
            id: Uuid::from_str(self.id.as_str()).unwrap(),
            created_at: self.created_at.naive_utc(),
            tweet_id,
        }
    }
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = likes)]
pub struct Likes {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub tweet_id: Uuid,
}

impl Likes {
    pub fn to_like(&self) -> Like {
        Like {
            id: self.id.to_string(),
            created_at: Utc.from_utc_datetime(&self.created_at),
        }
    }
}
