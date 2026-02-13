#[derive(Debug)]
pub enum Command {
    Ping,
    Get { key: String },
    Set { key: String, val: String },
    Unknown
}

pub fn parse(line: &str) -> Command {
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    if parts.is_empty() {
        return Command::Unknown;
    }

    match parts[0].to_uppercase().as_str() {
        "PING" => Command::Ping,
        "GET" if parts.len() == 2 => Command::Get {
            key: parts[1].to_string()
        },
        "SET" if parts.len() >= 3 => Command::Set {
            key: parts[1].to_string(),
            val: parts[2..].join(" ")
        },
        _ => Command::Unknown,
    }
}