use anyhow::Result;
use sqlx::{SqlitePool, migrate::Migrator};

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn init_db(database_url: &str) -> Result<SqlitePool> {
    let pool = SqlitePool::connect(database_url).await?;

    MIGRATOR.run(&pool).await?;

    Ok(pool)
}
