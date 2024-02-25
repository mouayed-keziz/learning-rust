use std::io;
fn main() {
    println!("--- name and greating ---");

    println!("Please enter your name :");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");

    println!("Hello, {}!", name.trim());
}
