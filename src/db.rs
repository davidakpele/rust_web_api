use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn get_pg_pool(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to create PostgreSQL pool")
}
