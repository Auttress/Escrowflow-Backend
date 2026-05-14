use sqlx::{postgres::PgPoolOptions, PgPool};

pub type Db = PgPool;

pub async fn init_database(database_url: &str) -> Result<Db, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
}
