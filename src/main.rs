use std::io;

fn main() {
    println!("Taking input!");

    let mut input = String::new();
    let len;

    len = io::stdin()
        .read_line(&mut input)
        .expect("Reading line failed");

    println!("\nYou said \n{input}");
    println!("which is {} long", len - 1);
}
