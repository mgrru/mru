use std::{env, fs, sync::OnceLock};

use serde::Deserialize;

pub static MYSQL_CONFIG: OnceLock<Mysql> = OnceLock::new();
pub static SERVER_CONFIG: OnceLock<Server> = OnceLock::new();

#[derive(Debug, Deserialize)]
struct Config {
    mysql: Option<Mysql>,
    server: Option<Server>,
}

#[derive(Debug, Deserialize)]
pub struct Mysql {
    pub username: Option<String>,
    pub password: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub db_name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub ip: Option<String>,
    pub port: Option<u16>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            ip: None,
            port: None,
        }
    }
}

pub async fn use_config() -> Result<(), String> {
    let current_path = match env::current_dir() {
        Ok(p) => p,
        Err(_) => return Err("发生未知错误!->use_config()".to_string()),
    };
    let config_path = current_path.join("config.toml");
    let config_str = match fs::read_to_string(config_path) {
        Ok(str) => str,
        Err(_) => return Err("配置文件丢失,请检查config.toml文件!".to_string()),
    };
    let config: Config = toml::from_str(&config_str).unwrap();
    let mysql_config = match config.mysql {
        Some(mysql) => mysql,
        None => return Err("数据库未设置!".to_string()),
    };
    let server_config = match config.server {
        Some(server) => server,
        None => Server::new(),
    };

    MYSQL_CONFIG.get_or_init(|| mysql_config);
    SERVER_CONFIG.get_or_init(|| server_config);
    Ok(())
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;

    #[tokio::test]
    #[serial]
    async fn test_get_mysql_config() {
        let r = use_config().await;
        assert!(r.is_ok());

        let mysql_config = match MYSQL_CONFIG.get() {
            Some(obj) => obj,
            None => panic!("err"),
        };

        println!("{:?}", mysql_config);

        let server_config = match SERVER_CONFIG.get() {
            Some(obj) => obj,
            None => panic!("err"),
        };

        println!("{:?}", server_config);
    }
}
