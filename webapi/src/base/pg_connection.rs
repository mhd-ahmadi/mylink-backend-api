use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn create_pg_connection(db_url: &str) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await
        .expect("Database connection failed")
}