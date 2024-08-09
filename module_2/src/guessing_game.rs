// Import the necessary modules
use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn run() {
    println!("Lets play!!");   
    // Prompt for input
    println!("Guess the number:");
    // Initialize a mutable String variable to hold the guess
    let mut guess = String::new();

    // Read input from the user and handle any errors
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    // Print the user's guess
    println!("You guessed: {}", guess);
    let guess:u32=guess.trim().parse()
        .expect("Please type a number!");    
    let secret_number=rand::thread_rng().gen_range(1,101);
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You guessed correctly!"),
    }  
    println!("The secret number is : {}", secret_number);   
}

