use std::io::{self, Write, Read};
use std::net::TcpStream;

fn main() {
    let server_addr = "127.0.0.1:6379";
    loop {
        print!(">>> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let mut stream = match TcpStream::connect(server_addr) {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Error connecting to the server: {}", e);
                continue;
            }
        };

        if let Err(e) = send_request(&mut stream, input) {
            eprintln!("Error sending request: {}", e);
            continue;
        }

        let mut response = String::new();
        if let Err(e) = read_response(&mut stream, &mut response) {
            eprintln!("Error reading response: {}", e);
            continue;
        }

        println!("{}", response);
    }
}

fn send_request(stream: &mut TcpStream, request: &str) -> io::Result<()> {
    stream.write_all(request.as_bytes())?;
    Ok(())
}

fn read_response(stream: &mut TcpStream, response: &mut String) -> io::Result<()> {
    stream.read_to_string(response)?;
    Ok(())
}
