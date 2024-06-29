use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::{env, time::Duration};

pub mod user_mapper;

use std::sync::OnceLock;
static INIT_POOL: OnceLock<Pool<MySql>> = OnceLock::new();

pub async fn init_pool() -> Result<(), sqlx::Error> {
    // dotenv().ok();
    let url = match env::var("url") {
        Ok(url) => url,
        Err(_) => {
            return Err(sqlx::Error::Configuration("未设置url!".into()));
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
            ));
        }
    };

    INIT_POOL.get_or_init(|| pool);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_init_pool() {
        dotenv().ok();
        let r = init_pool().await;
        assert!(r.is_ok());
    }

    #[tokio::test]
    #[serial]
    async fn test_init_pool_missing_url() {
        dotenv().ok();
        env::remove_var("url"); // Remove the url environment variable

        let r = init_pool().await;

        assert!(r.is_err());

        assert_eq!(
            r.unwrap_err().to_string(),
            sqlx::Error::Configuration("未设置url!".into()).to_string()
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_init_pool_connect_failure() {
        dotenv().ok();
        // Set an incorrect or unreachable URL
        env::set_var("url", "mysql://user:password@localhost:9999/db_name");

        let r = init_pool().await;
        assert!(r.is_err());

        assert_eq!(
            r.unwrap_err().to_string(),
            sqlx::Error::Configuration("连接失败!请检查数据库设置是否正确!".into()).to_string()
        );
    }
}
