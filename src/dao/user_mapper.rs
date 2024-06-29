use crate::model::User;

use super::INIT_POOL;

pub async fn get_all_users() -> Result<Vec<User>, sqlx::Error> {
    let pool = match INIT_POOL.get() {
        Some(p) => p,
        None => return Err(sqlx::Error::Configuration("连接池丢失!".into())),
    };
    sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool)
        .await
}

#[cfg(test)]
mod tests {
    use crate::dao::init_pool;

    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_get_all_users() {
        dotenv().ok();
        let r = init_pool().await;
        assert!(r.is_ok());

        let users = match get_all_users().await {
            Ok(us) => us,
            Err(_) => vec![],
        };

        assert!(!users.is_empty());
    }
}
