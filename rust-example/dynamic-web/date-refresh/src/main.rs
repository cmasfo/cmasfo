
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
  let listener = TcpListener::bind("localhost:7878").unwrap();
  
  for stream in listener.incoming() {
    let stream = stream.unwrap();
    
    handle_connection(stream);
  }
}

fn handle_connection(stream: TcpStream) {
  let mut reader = BufReader::new(stream);
  let mut buffer = String::new();
  
  reader.read_line(&mut buffer).unwrap();
  
  let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
  
  let response = format!("HTTP/1.1 200 OK\r\n\r\n{now}");
  
  reader.get_mut().write(response.as_bytes()).unwrap();
  reader.get_mut().flush().unwrap();
}
