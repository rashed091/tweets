use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::like::Like;
use crate::schema::tweets;

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
    pub likes: Vec<Like>,
}

impl Tweet {
    pub fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            message,
            likes: vec![],
        }
    }

    pub fn to_tweet_db(&self) -> Tweets {
        Tweets {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            message: self.message.clone(),
        }
    }

    pub fn add_likes(&self, likes: Vec<Like>) -> Self {
        Self {
            id: self.id.clone(),
            created_at: self.created_at.clone(),
            message: self.message.clone(),
            likes,
        }
    }
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = tweets)]
pub struct Tweets {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub message: String,
}

impl Tweets {
    pub fn to_tweet(&self) -> Tweet {
        Tweet {
            id: self.id.to_string(),
            created_at: Utc.from_utc_datetime(&self.created_at),
            message: self.message.clone(),
            likes: vec![],
        }
    }
}
