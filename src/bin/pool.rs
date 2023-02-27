use fauxchat::twitch_api::UserPool;
use tokio::{fs::File, io::AsyncWriteExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = UserPool::get().await?;

    let pool_str = serde_json::to_string(&pool)?;

    File::create("pool.json")
        .await?
        .write_all(pool_str.as_bytes())
        .await?;

    Ok(())
}
