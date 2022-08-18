use std::io;  //bring in ability to accept user input

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); //mutable variable guess is bound to a new emply string

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
