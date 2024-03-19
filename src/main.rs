mod common;
mod controllers;
mod models;
mod schema;
mod services;

use controllers::{like, tweet};

use dotenv::dotenv;
use salvo::cors::Cors;
use salvo::http::Method;
use salvo::logging::Logger;
use salvo::prelude::*;
use std::env;

#[handler]
async fn health() -> &'static str {
    "I'm alive"
}

#[handler]
async fn index() -> &'static str {
    "Welcome to rustful-service"
}

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt().init();

    dotenv().ok();

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let cors = Cors::new()
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        .into_handler();

    let router = Router::with_hoop(cors)
        .hoop(Logger::new())
        .get(index)
        .push(Router::with_path("/healthz").get(health))
        .push(
            Router::with_path("/tweets")
                .get(tweet::list)
                .post(tweet::create)
                .push(
                    Router::with_path("<id>")
                        .get(tweet::find)
                        .delete(tweet::delete),
                )
                .push(
                    Router::with_path("<id>/likes")
                        .get(like::list)
                        .post(like::add)
                        .delete(like::remove),
                ),
        );

    let acceptor = TcpListener::new(server_url).bind().await;

    Server::new(acceptor).serve(router).await;
}
