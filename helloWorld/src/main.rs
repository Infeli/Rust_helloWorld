use std::io;

fn main() {
    println!("Hello, world!");
    println!("-------------");
    println!("Jak se jmenuješ?");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let name = name.trim();

    print!("Ahoj {name}, jak se daří?!");
}
