use std::io::{self, Read};
fn main() {
    let mut input = String::new();
   let stdin = io::stdin();
let mut buf = String::new();
stdin.read_line(&mut buf).ok();
    println!("{}",buf);
}