pub fn run(){
    //Print to console
    println!("hello, from the print.rs file");
    //Return a value
    println!("Number: {}",1);
    //basic formatting
    println!("{} is from {}","Mac","earth");

    //positional arguments
    println!("{0} is from {1} and  {0} likes {2}",
    "Mac","earth", "coding");

    //Named arguments
    println!("{name} likes to play {activity}",
    name = "Mac", activity = "chess");
    
    //placeholder traits
    println!("Binary: {:b} hex: {:x} octal: {:o}",10,10,10);

    //placeholder for debug traits
    println!("{:?}",(12,true,"hello world!"));

    //basic math
    println!("10 +10 ={}",10 +10);
}