
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {

  let listener = TcpListener::bind("localhost:7878").unwrap();

  // compile rust-wasm project
  // project is located at ./rust-wasm
  println!("Compiling rust-wasm project...");
  match std::process::Command::new("wasm-pack")
  .arg("build")
  .arg("--target")
  .arg("web")
  .arg("--out-dir")
  .arg("pkg")
  .current_dir("./rust-wasm")
  .status() { // check if compilation is successful
    Ok(status) => {
      if !status.success() {
        println!("Compilation failed.");
        std::process::exit(1);
      }
    },
    Err(_) => {
      println!("Compilation failed.");
      std::process::exit(1);
    }
  }
  println!("Compiling is finished.");
  
  println!("Running server on localhost:7878...");

  for stream in listener.incoming() {
    let stream = stream.unwrap();
    
    handle_connection(stream);
  }

}

// wasm version
fn handle_connection(stream: TcpStream) {
  let mut reader = BufReader::new(stream);
  let mut buffer = String::new();
  
  reader.read_line(&mut buffer).unwrap();
  
  let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

  let request_path = buffer.split_whitespace().nth(1).unwrap();

  let response;

  if request_path == "/time" {
    response = format!("HTTP/1.1 200 OK\r\n\r\n{now}");
    reader.get_mut().write(response.as_bytes()).unwrap();
    reader.get_mut().flush().unwrap();
  // send wasm files
  } else if request_path == "/rust_wasm_bg.wasm" {
    let mut file = std::fs::File::open("rust-wasm/pkg/rust_wasm_bg.wasm").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/wasm\r\n\r\n");
    reader.get_mut().write(response.as_bytes()).unwrap();
    reader.get_mut().write(&contents).unwrap();
    reader.get_mut().flush().unwrap();
  } else if request_path == "/rust_wasm.js" {
    let mut file = std::fs::File::open("rust-wasm/pkg/rust_wasm.js").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/javascript\r\n\r\n");
    reader.get_mut().write(response.as_bytes()).unwrap();
    reader.get_mut().write(&contents).unwrap();
    reader.get_mut().flush().unwrap();
  } else {
    response = format!( // update time button
      "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\n\r\n
      <html>
        <head>
          <script type='module'>
            import init from './rust_wasm.js';
            init();
          </script>
        </head>
        <body>
        </body>
      </html>
    ");
    reader.get_mut().write(response.as_bytes()).unwrap();
    reader.get_mut().flush().unwrap();
  }
}

// js version
/*
fn handle_connection(stream: TcpStream) {
  let mut reader = BufReader::new(stream);
  let mut buffer = String::new();
  
  reader.read_line(&mut buffer).unwrap();
  
  let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

  let request_path = buffer.split_whitespace().nth(1).unwrap();

  let response;

  if request_path == "/time" {
    response = format!("HTTP/1.1 200 OK\r\n\r\n{now}");
  } else {
    response = format!(
      "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\n\r\n
      <html>
        <body>
          <p id='time'>{now}</p>
          <button onclick='updateTime()'>Update time</button>
          <script>
            function updateTime() {{
              let xhr = new XMLHttpRequest();
              xhr.open('GET', '/time');
              xhr.send();
              xhr.onload = function() {{
                document.getElementById('time').innerHTML = xhr.responseText;
              }}
            }}
          </script>
        </body>
      </html>"
    );
  }

  reader.get_mut().write(response.as_bytes()).unwrap();
  reader.get_mut().flush().unwrap();
}
*/
