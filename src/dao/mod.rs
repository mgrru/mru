use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::time::Duration;

pub mod user_mapper;

use std::sync::OnceLock;

use crate::config::MYSQL_CONFIG;

static INIT_POOL: OnceLock<Pool<MySql>> = OnceLock::new();

pub async fn init_pool() -> Result<(), sqlx::Error> {
    // dotenv().ok();
    let url = match get_url().await {
        Ok(url) => url,
        Err(err) => {
            return Err(sqlx::Error::Configuration(err.into()));
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

    INIT_POOL.get_or_init(|| pool);
    Ok(())
}

async fn get_url() -> Result<String, String> {
    let mysql_config = match MYSQL_CONFIG.get() {
        Some(mysql) => mysql,
        None => return Err("配置丢失!".to_string()),
    };

    let url = match &mysql_config.url {
        Some(u) => u,
        None => &String::new(),
    };

    if !url.is_empty() {
        return Ok(url.clone());
    }

    let username = match &mysql_config.username {
        Some(uname) => uname,
        None => return Err("数据库未设置!".into()),
    };

    let password = match &mysql_config.password {
        Some(pass) => pass,
        None => return Err("数据库未设置!".into()),
    };

    let host = match &mysql_config.host {
        Some(host) => host,
        None => return Err("数据库未设置!".into()),
    };
    let port = match &mysql_config.port {
        Some(port) => port,
        None => &3306,
    };

    let db_name = match &mysql_config.db_name {
        Some(db_name) => db_name,
        None => return Err("数据库未设置!".into()),
    };

    Ok(format!(
        "mysql://{}:{}@{}:{}/{}",
        username, password, host, port, db_name
    ))
}

#[cfg(test)]
mod tests {
    use std::env::{remove_var, set_var};

    use super::*;
    use dotenv::dotenv;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_get_url() {
        let r = get_url().await;
        assert!(r.is_ok());

        let url = match r {
            Ok(u) => u,
            Err(err) => err.to_string(),
        };

        println!("{}", url);
    }

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
        remove_var("sqlx.database.url");

        let r = init_pool().await;
        assert!(r.is_ok());
    }

    #[tokio::test]
    #[serial]
    async fn test_init_pool_missing_dbset() {
        // dotenv().ok();
        remove_var("sqlx.database.url");
        remove_var("sqlx.database.username");
        remove_var("sqlx.database.password");
        remove_var("sqlx.database.ip");
        remove_var("sqlx.database.db_name");

        let r = init_pool().await;
        assert!(r.is_err());
        assert_eq!(
            r.unwrap_err().to_string(),
            sqlx::Error::Configuration("连接失败!请检查数据库设置是否正确!".into()).to_string()
        )
    }

    #[tokio::test]
    #[serial]
    async fn test_init_pool_connect_failure() {
        dotenv().ok();
        set_var(
            "sqlx.database.url",
            "mysql://user:password@localhost:9999/db_name",
        );

        let r = init_pool().await;
        assert!(r.is_err());
        assert_eq!(
            r.unwrap_err().to_string(),
            sqlx::Error::Configuration("连接失败!请检查数据库设置是否正确!".into()).to_string()
        );
    }
}
