use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

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

  println!("Received request: {}", request);

  let json_response = r#"{"message": "Hello"}"#;
  let response = format!(
    "HTTP/1.1 200 OK\r\n\
       Content-Type: application/json\r\n\
       Content-Length: {}\r\n\
       \r\n\
       {}",
    json_response.len(),
    json_response
  );

  if let Err(e) = stream.write_all(response.as_bytes()) {
    eprintln!("Failed to send response: {}", e);
  }
}

fn main() {
  let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
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
