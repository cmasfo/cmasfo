
fn main() {
  
}

pub fn flush() {
  use std::io::Write;
  std::io::stdout().flush().unwrap();  
}

pub fn get_line() -> String {
  let mut s = String::new();
  std::io::stdin()
  .read_line(&mut s).unwrap();
  s.trim().to_string()
}

#[macro_export]
macro_rules! printfl {

}
