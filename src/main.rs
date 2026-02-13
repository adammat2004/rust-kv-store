mod db;
mod protocol;
mod server;

use anyhow::Result;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let db= Arc::new(db::Db::new());
    server::run("127.0.0.1:6379", db).await
}
