pub mod net;

use std::io::{Read, Write};
use std::net::TcpStream;

use crate::net::tcp::TcpListenerMyself;

fn handle_client(mut stream: TcpStream) {
  let mut buffer = [0; 1024];
  let mut request = String::new();

  loop {
    match stream.read(&mut buffer) {
      Ok(0) => break,
      Ok(n) => {
        request.push_str(&String::from_utf8_lossy(&buffer[..n]));
        if request.contains("\r\n\r\n") {
          break;
        }
      }
      Err(e) => {
        eprintln!("Failed to read from connection: {}", e);
        return;
      }
    }
  }

  let parts: Vec<&str> = request.lines().next().unwrap_or("").split_whitespace().collect();

  if parts.len() < 2 {
    let json_response = r#"{"message": "Bad Request"}"#;
    send_response(&mut stream, "400 Bad Request", "application/json", json_response);
    return;
  }

  match parts[1] {
    "/" => {
      let json_response = r#"{"message": "Welcome!"}"#;
      send_response(&mut stream, "200 OK", "application/json", json_response);
    }
    "/hello" => {
      let json_response = r#"{"message": "Hello, World!"}"#;
      send_response(&mut stream, "200 OK", "application/json", json_response);
    }
    _ => {
      let json_response = r#"{"message": "Not Found"}"#;
      send_response(&mut stream, "404 Not Found", "application/json", json_response);
    }
  }
}

fn send_response(stream: &mut TcpStream, status: &str, content_type: &str, body: &str) {
  let response = format!(
    "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
    status,
    content_type,
    body.len(),
    body
  );

  if let Err(e) = stream.write_all(response.as_bytes()) {
    eprintln!("Failed to send response: {}", e);
  }
}

fn main() {
  let listener = TcpListenerMyself::bind("0.0.0.0:8000").unwrap();
  println!("Server listening. Visit http://localhost:8000");

  let local_addr = listener.local_addr().unwrap();
  println!("Local address: {}", local_addr);

  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        std::thread::spawn(|| {
          handle_client(stream);
        });
      }
      Err(e) => {
        eprintln!("Error: {}", e);
      }
    }
  }
}
