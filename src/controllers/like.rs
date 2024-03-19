use salvo::prelude::*;
use uuid::Uuid;

use crate::common::db::Database;
use crate::services::like::LikeService;

/// list last 50 likes from a tweet `/tweets/{id}/likes`
#[handler]
pub async fn list(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    let id = req.param::<Uuid>("id").unwrap();
    let mut connection = Database::new().get_connection();
    let likes = LikeService::list_likes(id, &mut connection);
    res.render(Json(likes.unwrap()));
}

/// add one like to a tweet `/tweets/{id}/likes`
#[handler]
pub async fn add(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    let id = req.param::<Uuid>("id").unwrap();
    let mut connection = Database::new().get_connection();

    let like = LikeService::create_like(id, &mut connection);
    res.render(Json(like.unwrap()));
}

/// remove one like from a tweet `/tweets/{id}/likes`
#[handler]
pub async fn remove(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    // in any case return status 204
    let id = req.param::<Uuid>("id").unwrap();
    let mut connection = Database::new().get_connection();

    let like = LikeService::delete_like(id, &mut connection);
    res.render(Json(like.unwrap()));
}
