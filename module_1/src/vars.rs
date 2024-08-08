pub fn run(){
    //varables are immutable by default
    let name="Mac";//string
    let age=37;//integer
    println!("My name is {} and i am {}", name, age);
    let age=23;
    println!("My age is {} and i am {}",age,name);

    //Define constant
    const ID: i32=001;//constant
    println!("ID: {}",ID);

    //assign multiple variables
    let (my_name, my_age) =("Mac", 23);
    println!("My name is {} and i am {}", my_name, my_age);
}