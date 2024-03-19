use salvo::prelude::*;
use uuid::Uuid;

use crate::common::db::Database;
use crate::models::tweet::Tweet;
use crate::services::tweet::TweetService;

/// list 50 last tweets `/tweets`
#[handler]
pub async fn list(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    let mut connection = Database::new().get_connection();
    let tweets = TweetService::list_tweets(50, &mut connection);

    res.render(Json(tweets.unwrap()));
}

/// create a tweet `/tweets`
#[handler]
pub async fn create(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    let mut connection = Database::new().get_connection();
    let new_tweet = Tweet::new("testing".to_string());
    let tweet = TweetService::create_tweet(new_tweet, &mut connection);

    res.render(Json(tweet.unwrap()));
}

/// find a tweet by its id `/tweets/{id}`
#[handler]
pub async fn find(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    let id = req.param::<Uuid>("id").unwrap();
    let mut connection = Database::new().get_connection();

    let tweets = TweetService::find_tweet(id, &mut connection);
    res.render(Json(tweets.unwrap()));
}

/// delete a tweet by its id `/tweets/{id}`
#[handler]
pub async fn delete(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    // in any case return status 204
    let id = req.param::<Uuid>("id").unwrap();
    let mut connection = Database::new().get_connection();

    let tweets = TweetService::delete_tweet(id, &mut connection);
    res.render(Json(tweets.unwrap()));
}
