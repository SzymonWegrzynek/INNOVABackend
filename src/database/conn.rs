use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

pub struct DatabaseConn {
    pool: PgPool,
}

impl DatabaseConn {
    pub async fn create_pool() -> Self {
        let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool: PgPool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&db_url)
            .await
            .unwrap();

        DatabaseConn { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
