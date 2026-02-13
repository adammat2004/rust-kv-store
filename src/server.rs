use crate::db::Db;
use crate::protocol::{parse, Command};
use anyhow::Result;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

pub async fn run(addr: &str, db: Arc<Db>) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    println!("redis_clone listing on {addr}");

    loop {
        let (socket, peer) = listener.accept().await?;
        println!("client connected: {peer}");

        let db = db.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, db).await {
                eprintln!("connection error: {e}");
            }
        });
    }
}

async fn handle_connection(stream: TcpStream, db: Arc<Db>) -> Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    writer.write_all(b"Welcome to redis_clone. Try: PING | SET a 1 | GET a\n").await?;

    loop {
        line.clear();
        let n = reader.read_line(&mut line).await?;
        if n == 0 {
            // client closed connection
            return Ok(());
        }

        match parse(&line) {
            Command::Ping => {
                writer.write_all(b"Pong\n").await?;
            }
            Command::Set { key, val } => {
                db.set(key, val).await;
                writer.write_all(b"OK\n").await?;
            }
            Command::Get { key } => match db.get(&key).await {
                Some(v) => {
                    writer.write_all(v.as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                }
                None => {
                    writer.write_all(b"(nil)\n").await?;
                }
            },
            Command::Unknown => {
                writer.write_all(b"ERR unknown command\n").await?;
            }
        }

        return Ok(());
    }
}