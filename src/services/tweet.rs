use uuid::Uuid;
use diesel::{ExpressionMethods, RunQueryDsl};
use diesel::query_dsl::methods::{FilterDsl, OrderDsl, LimitDsl};
use diesel::result::Error;

use crate::common::db::DBPooledConnection;
use crate::common::response::Response;
use crate::models::tweet::{Tweet, Tweets};

pub type ResponseTweets = Response<Tweet>;

pub struct TweetService;

impl TweetService {
    pub fn list_tweets(total_tweets: i64, conn: &mut DBPooledConnection) -> Result<ResponseTweets, Error> {
        use crate::schema::tweets::dsl::*;

        let _tweets = match tweets
            .order(created_at.desc())
            .limit(total_tweets)
            .load::<Tweets>(conn)
        {
            Ok(tws) => tws,
            Err(_) => vec![],
        };

        Ok(ResponseTweets {
            results: _tweets
                .into_iter()
                .map(|t| t.to_tweet())
                .collect::<Vec<Tweet>>(),
        })
    }

    pub fn find_tweet(_id: Uuid, conn: &mut DBPooledConnection) -> Result<Tweet, Error> {
        use crate::schema::tweets::dsl::*;

        let res = tweets.filter(id.eq(_id)).load::<Tweets>(conn);
        match res {
            Ok(tweets_db) => match tweets_db.first() {
                Some(tweet_db) => Ok(tweet_db.to_tweet()),
                _ => Err(Error::NotFound),
            },
            Err(err) => Err(err),
        }
    }

    pub fn create_tweet(tweet: Tweet, conn: &mut DBPooledConnection) -> Result<Tweet, Error> {
        use crate::schema::tweets::dsl::*;

        let tweet_db = tweet.to_tweet_db();
        let _ = diesel::insert_into(tweets).values(&tweet_db).execute(conn);

        Ok(tweet_db.to_tweet())
    }

    pub fn delete_tweet(_id: Uuid, conn: &mut DBPooledConnection) -> Result<(), Error> {
        use crate::schema::tweets::dsl::*;

        let res = diesel::delete(tweets.filter(id.eq(_id))).execute(conn);
        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}
