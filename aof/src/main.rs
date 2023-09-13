use serde::{Deserialize, Serialize};
use tokio::fs::OpenOptions;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[derive(Debug, Serialize, Deserialize)]
enum Command {
    Set { key: String, value: String },
    Del { key: String },
}

impl Command {
    fn to_string(&self) -> String {
        match self {
            Command::Set { key, value } => format!("SET {} {}\n", key, value),
            Command::Del { key } => format!("DEL {}\n", key),
        }
    }

    fn from_string(s: &str) -> Option<Command> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() < 2 {
            return None;
        }
        let cmd = match parts[0] {
            "SET" => {
                if parts.len() >= 3 {
                    Command::Set {
                        key: parts[1].to_string(),
                        value: parts[2..].join(" "),
                    }
                } else {
                    return None;
                }
            }
            "DEL" => {
                Command::Del {
                    key: parts[1].to_string(),
                }
            }
            _ => return None,
        };
        Some(cmd)
    }
}

async fn write_command_to_aof(command: &Command) -> Result<(), tokio::io::Error> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.aof")
        .await?;

    let cmd_string = command.to_string();
    file.write_all(cmd_string.as_bytes()).await?;

    Ok(())
}

async fn recover_from_aof() -> Result<Vec<Command>, tokio::io::Error> {
    let file = File::open("log.aof").await?;
    let reader = BufReader::new(file);
    let mut commands = Vec::new();

    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await? {
        if let Some(cmd) = Command::from_string(&line) {
            commands.push(cmd);
        }
    }

    Ok(commands)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example: Write commands to AOF
    let set_command = Command::Set {
        key: "name".to_string(),
        value: "Alice".to_string(),
    };
    let set_command2 = Command::Set {
        key: "name".to_string(),
        value: "Bob".to_string(),
    };
    let del_command = Command::Del {
        key: "name".to_string(),
    };

    write_command_to_aof(&set_command).await?;
    write_command_to_aof(&set_command2).await?;
    write_command_to_aof(&del_command).await?;

    // Example: Recover commands from AOF
    let recovered_commands = recover_from_aof().await?;
    println!("Recovered commands: {:?}", recovered_commands);
    
    for cmd in recovered_commands {
        match cmd {
            Command::Set { key, value } => {
                println!("SET {} {}", key, value);
            }
            Command::Del { key } => {
                println!("DEL {}", key);
            }
        }
    }

    write_command_to_aof(&set_command).await?;
    write_command_to_aof(&set_command2).await?;
    write_command_to_aof(&del_command).await?;

    Ok(())
}
