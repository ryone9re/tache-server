use anyhow::Result;
use sqlx::mysql::MySqlPool;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = MySqlPool::connect("mysql://root:@127.0.0.1:14000/test?charset=utf8mb4").await?;

    let row: (i64,) = sqlx::query_as("SELECT ?")
        .bind(150)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);

    Ok(())
}
