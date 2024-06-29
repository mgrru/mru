use axum::{routing::{get, post}, Router};

use crate::dao;

pub async fn get_routers() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", get(get_all_user))
        .route("/users", post(get_all_user_string))
}

async fn root() -> &'static str {
    "ru"
}

async fn get_all_user() -> String {
    let users = match dao::user_mapper::get_all_users().await {
        Ok(us) => us,
        Err(err) => return err.to_string(),
    };
    serde_json::to_string(&users).unwrap()
}

async fn get_all_user_string() -> String {
    let users = match dao::user_mapper::get_all_users().await {
        Ok(us) => us,
        Err(err) => return err.to_string(),
    };
    serde_json::to_string(&users).unwrap()
}