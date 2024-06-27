use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::{env, time::Duration};
use tracing::error;

pub mod user_mapper;

use std::sync::OnceLock;
static INIT_POOL: OnceLock<Pool<MySql>> = OnceLock::new();

pub async fn init_pool() -> Result<Pool<MySql>, sqlx::Error> {
    // dotenv().ok();
    let url = match env::var("url") {
        Ok(url) => url,
        Err(_) => {
            error!("获取db url失败!");
            return Err(sqlx::Error::Configuration("未设置url".into()));
        }
    };

    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .idle_timeout(Duration::from_secs(30))
        .acquire_timeout(Duration::from_secs(10))
        .connect(&url)
        .await
    {
        Ok(pool) => pool,
        Err(_) => {
            return Err(sqlx::Error::Configuration(
                "连接失败!请检查数据库设置是否正确!".into(),
            ))
        }
    };

    INIT_POOL.get_or_init(|| pool.clone());
    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_init_pool() {
        dotenv().ok();
        let pool = init_pool().await;
        assert!(pool.is_ok(), "Failed to initialize the pool");
    }

    #[tokio::test]
    async fn test_init_pool_missing_url() {
        env::remove_var("url"); // Remove the url environment variable

        let result = init_pool().await;
        // assert_eq!(result.is_ok(), true);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            sqlx::Error::Configuration("未设置url".into()).to_string()
        );
    }

    #[tokio::test]
    async fn test_init_pool_connect_failure() {
        // Set an incorrect or unreachable URL
        env::set_var("url", "mysql://user:password@localhost:9999/db_name");

        let result = init_pool().await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            sqlx::Error::Configuration("连接失败!请检查数据库设置是否正确!".into()).to_string()
        );
    }
}
