use std::io;
use chrono::Local;

fn main() {
    println!("Please enter your name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim();
    println!("Hello, {}! Welcome to Rust programming!", name);
    let current_date = Local::now();
    println!("Today's date is: {}", current_date.format("%Y-%m-%d"));
}