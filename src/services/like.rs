use uuid::Uuid;
use diesel::{ExpressionMethods, RunQueryDsl};
use diesel::query_dsl::methods::{FilterDsl, OrderDsl};
use diesel::result::Error;

use crate::common::db::DBPooledConnection;
use crate::common::response::Response;
use crate::models::like::{Like, Likes};

pub type ResponseLikes = Response<Like>;

pub struct LikeService;

impl LikeService {
    pub fn list_likes(_tweet_id: Uuid, conn: &mut DBPooledConnection) -> Result<ResponseLikes, Error> {
        use crate::schema::likes::dsl::*;

        let _likes: Vec<Likes> = match likes
            .filter(tweet_id.eq(_tweet_id))
            .order(created_at.desc())
            .load::<Likes>(conn)
        {
            Ok(lks) => lks,
            Err(_) => vec![],
        };

        Ok(ResponseLikes {
            results: _likes
                .into_iter()
                .map(|l| l.to_like())
                .collect::<Vec<Like>>(),
        })
    }

    pub fn create_like(
        _tweet_id: Uuid,
        conn: &mut DBPooledConnection,
    ) -> Result<Like, Error> {
        use crate::schema::likes::dsl::*;

        let like = Like::new();
        let _ = diesel::insert_into(likes)
            .values(like.to_like_db(_tweet_id))
            .execute(conn);

        Ok(like)
    }

    pub fn delete_like(_tweet_id: Uuid, conn: &mut DBPooledConnection) -> Result<(), Error> {
        use crate::schema::likes::dsl::*;

        let res = diesel::delete(likes.filter(id.eq(_tweet_id))).execute(conn);
        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}
