mod banner;
mod dao;
mod model;
mod utils;

use axum::{routing::get, Router};
use banner::banner;

use dao::init_pool;
use dotenv::dotenv;
use utils::my_log;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // 加载日志组件
    my_log::init_log();

    // 打印
    banner();

    // 加载环境变量
    dotenv().ok();

    init_pool().await?;

    let app = Router::new().route("/", get(root));
    // .route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9901").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn root() -> &'static str {
    let users = match dao::user_mapper::get_all_users().await {
        Ok(us) => us,
        Err(_) => return "404",
    };
    let res = serde_json::to_string(&users).unwrap();
    Box::leak(res.into_boxed_str())
}