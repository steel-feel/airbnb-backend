use sqlx::{Pool, Postgres};

pub async fn init_db() -> Result<Pool<Postgres>, sqlx::Error> {
    let url = std::env::var("DATABASE_URL").unwrap();
    let pool = sqlx::PgPool::connect(&url).await?;
    Ok(pool)
}
