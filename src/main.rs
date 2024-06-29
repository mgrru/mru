mod banner;
mod config;
mod dao;
mod model;
mod utils;
mod web;

use banner::banner;

use config::{use_config, Server, SERVER_CONFIG};
use dao::init_pool;
use tracing::info;
use utils::my_log;
use web::user_controller::get_routers;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // 加载日志组件
    my_log::init_log();

    // 打印
    banner();

    // 加载环境变量
    match use_config().await {
        Ok(_) => {}
        Err(err) => return Err(sqlx::Error::Configuration(err.into())),
    };

    init_pool().await?;

    let app = get_routers().await;

    let server_config = match SERVER_CONFIG.get() {
        Some(server) => server,
        None => &Server::new(),
    };

    let ip = match &server_config.ip {
        Some(i) => i,
        None => &String::from("localhost"),
    };

    let port = match &server_config.port {
        Some(p) => p,
        None => &9901,
    };

    let addr = format!("{}:{}", ip, port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    info!("server is running on {}", addr);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
