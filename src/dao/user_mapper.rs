use crate::model::User;

use super::INIT_POOL;

pub async fn get_all_users() -> Result<Vec<User>, sqlx::Error> {
    let pool = match INIT_POOL.get() {
        Some(p) => p,
        None => return Err(sqlx::Error::Configuration("数据库未连接!".into())),
    };
    sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool)
        .await
}
