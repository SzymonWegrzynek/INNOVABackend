#[cfg(test)]
mod tests {
    use crate::database::conn::DatabaseConn;
    use dotenv::dotenv;
    use sqlx::query;
    use std::env;

    #[tokio::test]
    async fn test_create_pool() {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL");
        assert!(db_url.is_ok(), "DATABASE_URL must be set");

        let db_conn = DatabaseConn::create_pool().await;
        let result = query("SELECT 1").fetch_one(db_conn.pool()).await;
        assert!(result.is_ok(), "failed to create conn pool")
    }
}
