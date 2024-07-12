use std::io;  // For IO functionality

fn main() {
    println!("Please enter a guess.");
    println!("> Input the number:");

    let mut guess = String::new();  // creates an empty string instance

    io::stdin()  // creates an instance of std::io::Stdin()
        .read_line(&mut guess)
        .expect("Failed to read the input!");
}
