use std::env;
use std::io::Write;
// use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

use mini_redis::{client::*, printresp};

#[volo::main]
async fn main() {

    let args: Vec<String> = env::args().collect();

    // Check args to set host and port
    match set_port(&args) {
        Ok(_) => {},
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    set("test_key", "test_value").await;

    loop {
        print!(">>> ");
        let _ = std::io::stdout().flush();  // flush stdout

        // Read input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        // Deal with input
        let input: Vec<&str> = input.trim()
                                    .split(" ")
                                    .filter(|s| !s.is_empty())
                                    .collect();
        // Check input is not empty
        if input.is_empty() {
            continue;
        }
        // Match input to command
        match input[0].to_uppercase().as_str() {
            "GET" => {
                if input.len() != 2 {
                    println!("ERROR: Invalid arguments for GET, please specify a key");
                    continue;
                }
                let key = input[1];
                let value = get(key).await;
                printresp!(value);
            },
            "SET" => {
                if input.len() != 3 && input.len() != 4 {
                    println!("ERROR: wrong number of arguments for command");
                    continue;
                }
                let value = match input.len() == 3 {
                    true => {
                        let key = input[1];
                        let value = input[2];
                        let value = set(key, value).await;
                        value
                    },
                    false => {
                        let key = input[1];
                        let value = input[2];
                        let ex = input[3];
                        let value = set_expired(key, value, ex).await;
                        value
                    },
                };
                printresp!(value);
            },
            "DEL" => {
                if input.len() != 2 {
                    println!("ERROR: Invalid arguments for DEL, please specify a key");
                    continue;
                }
                let key = input[1];
                let value = del(key).await;
                printresp!(value);
            },
            "PING" => {
                let mut to_ping = "".to_string();
                if input.len() >= 2 {
                    for i in 1..input.len() {
                        if i != 1 {
                            to_ping += " ";
                        }
                        to_ping += input[i];
                    }
                } else {
                    to_ping += "PONG";
                };
                let value = ping(to_ping.as_str()).await;
                printresp!(value);
            },
            "SUBSCRIBE" => {
                if input.len() != 2 {
                    println!("ERROR: Invalid arguments for SUBSCRIBE, please specify only one channel");
                    continue;
                }
                let channel = input[1];
                println!("1) \"subscribe\"\n2) \"{}\"\n3) (integer) 1", channel);
                let message = subscribe(channel).await;
                match message {
                    Some(message) => println!("1) \"message\"\n2) \"{}\"\n3) \"{}\"", channel, message),
                    None => panic!("Invalid message"),
                }
            },
            "PUBLISH" => {
                if input.len() != 3 {
                    println!("(error) ERR wrong number of arguments for command");
                    continue;
                }
                let channel = input[1];
                let message = input[2];
                let message = publish(channel, message).await;
                printresp!(message);

            },
            "QUIT" => {
                println!("OK");
                break;
            },
            _ => {
                println!("ERROR: unknown command '{}'", input[0]);
            },
        }
    }
}