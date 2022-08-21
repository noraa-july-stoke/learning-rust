use std::io; // Brings io (input/output) from std library into scope.

// fn is used to declare a function in rust

fn main() {    // The main function is the etry into the rest of program.
    println!("Guess the number!");  // println! --> prints text to user console.
    println!("Please enter a guess: ");

    let mut guess = String::new(); // let declares variable, mut is optional &
//                   removes rust's default immutability of variables.
//                   the variable declared is empty instance of String type

//  In Rust, a string is a growable, utf-8 encoded bit of text.

//  The :: syntax in String::new() indicates new() is an associated function of
//  the string type.



    io::stdin()
    // Calls stdin() function from io. stdin() handles user input
    // Associated function- function that's implemented on a type.

    .read_line(&mut guess) // Calls read line method on standard input with
//  &mut guess as an argument to tell it which variable to store the input
//  inside. read_line takes whatever user tpyes and append it into string
//  without overwriting its contents. The string needs to be mutable so that
//  read_line funciton can mutate it. The & indicates argument is a referene
//  mut needs to be used again since references are immutable by default.


    .expect("Failed to read line");

    println!("You guessed: {guess}");

}
