std::io
pub fn run(){
    println!("Guess the number!");
    println!("Number:");
    let mut guess=String::new();
    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
    println!("You guessed:{}", guess);

}
   