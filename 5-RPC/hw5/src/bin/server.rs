use std::collections::HashMap;
use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Failed to bind to port 6379");
    let mut storage = HashMap::new();

    println!("Mini-Redis server is running on port 6379...");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];
                let bytes_read = stream.read(&mut buffer);
                
                let request = String::from_utf8_lossy(&buffer[..bytes_read.unwrap()]).to_string();
                println!("Received request: {} from {}", request, stream.peer_addr().unwrap());
                let response = handle_request(&request, &mut storage);

                stream.write_all(response.as_bytes()).expect("Failed to write response");
                stream.flush().expect("Failed to flush stream");
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn handle_request(request: &str, storage: &mut HashMap<String, String>) -> String {
    let mut parts = request.split_whitespace();
    match &parts.next().unwrap().to_uppercase()[..] {
        "GET" => {
            if let Some(key) = parts.next() {
                if let Some(value) = storage.get(key) {
                    return format!("GET: {:?}\r\n", value);
                }
            }
            return "ERROR: Key not found\r\n".to_string();
        }
        "SET" => {
            if let Some(key) = parts.next() {
                if let Some(value) = parts.next() {
                    let _ = storage.insert(key.to_string(), value.to_string());
                    return format!("SUCCESSFULLY SET {{ {:?}: {:?} }}\r\n", key, value);
                }
            }
            return "ERROR: Invalid arguments\r\n".to_string();
        }
        "DEL" => {
            if let Some(key) = parts.next() {
                if let Some(del_v) = storage.get(key) {
                    let del_v = del_v.clone().to_string();
                    storage.remove(key);
                    return format!("SUCCESSFULLY DEL {{ {:?}: {:?} }}\r\n", key, del_v);
                }
            }
            return "ERROR: Invalid arguments\r\n".to_string();
        }
        "PING" => {
            let mut echo = "".to_string();
            while let Some(arg) = parts.next() {
                echo += " ";
                echo += arg;
            }
            if echo.is_empty() {
                return "PONG\r\n".to_string();
            }
            return echo + "\r\n";
        }
        _ => {
            return "ERROR: Invalid command\r\n".to_string();
        }
    }
}
