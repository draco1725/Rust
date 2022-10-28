//lets see how we use a library in rust

use std::io;  

fn main() {
    let mut draco = String::new();
    println!("Enter your message");
    io::stdin().read_line(&mut draco);
    println!("Draco says {}", draco);

}
