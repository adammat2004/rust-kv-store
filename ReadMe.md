# Rust Key Value Store

A concurrent in-memory key–value store written in Rust using the Tokio async runtime.

This project implements a TCP server that supports multiple simultaneous clients and provides basic Redis-style commands over a custom text protocol.

---

##  Features

- Async TCP server (Tokio)
- Concurrent client handling (`tokio::spawn`)
- Shared in-memory state (`Arc<RwLock<HashMap<_, _>>>`)
- Simple command parsing
- Lightweight and fast
- Zero external database dependencies

---

##  Architecture

- **Networking:** `tokio::net::TcpListener`
- **Concurrency model:** async tasks (not OS threads)
- **Shared state:** `Arc<Db>` with `RwLock`
- **Storage:** In-memory `HashMap<String, String>`

The server follows a basic event-driven model:

Client → Parse Command → Execute → Respond

---

##  Supported Commands

| Command | Description |
|----------|-------------|
| `PING` | Returns `PONG` |
| `SET key value` | Stores a value |
| `GET key` | Retrieves a value |

Example:

```
PING
SET name Adam
GET name
```

---

##  Running the Project

### 1. Clone the repository

```bash
git clone https://github.com/adammat2004/rust-kv-store.git
```

### 2. Run the server

```bash
cargo run
```

Server listens on:

127.0.0.1:6379

### 3. Connect via TCP

Using `telnet` or similar:

```bash
telnet 127.0.0.1 6379
```