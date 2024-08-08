/*
Primitive types--
Integers: u8,i8,u16,i16,u32,i32,u64,i64,u128,i128,(number of bits they take in memory)
Floats:f32,f64
Boolean (bool)
Character(char)
Tuples
Arrays 

pub fn run(){
    //Default is i32
    let x=1;

    //Default is f64
    let y=2.5;
    
    //add explicit type
    let a: i64=2345431267;

    //find max size
    println!("Max i32: {}",std::i32::MAX);
    println!("Max i64: {}",std::i64::MAX);

    //boolean
    let is_active=true;
    println!("Is active: {}",is_active);
    println!("{:?}",(x,y,a,is_active));

    //get boolean from expression
    let is_greater=true;
    println!("10 is greater than 1 is {}",is_greater);
} 

pub fn run() {
    //mutable variables
    let mut x=1;
    println!("Hello, world! x={}", x);
    x=6;
    println!("x is now {}", x);

    //shadowing
    let a=1;
    let a=a + 6;
    let a=a + a +1;
    println!("the sum is now {}",a);

    let spaces="winner";
    let spaces=spaces.len();
    println!("The length of the string is {}",spaces);
}


pub fn run() {
    // Scalar data types
    let integer: i32 = 10; // 32-bit integer
    let float: f64 = 3.14; // 64-bit floating point
    let boolean: bool = true; // boolean value
    let name: char = 'A'; // character type

    // Print the values
    println!("Integer: {}", integer);
    println!("Float: {}", float);
    println!("Boolean: {}", boolean);
    println!("Character: {}", name);
}

pub fn run(){
    //numerical operations

    //addition
    let sum=5+10;
    println!("Sum: {}",sum);

    //subtraction
    let difference=10-5;
    println!("Difference: {}",difference);

    //multiplication
    let product=5*10;
    println!("Product: {}",product);

    //division
    let quotient=10/5;
    println!("Quotient: {}",quotient);

    //remainder
    let remainder=10%3;
    println!("Remainder: {}",remainder);

}
*/

//compound types
pub fn run(){
    //tuples
    let person=("John",30);
    println!("Person: {:?}",person);
    let tup: (i32, f64, i32)= (500,6.4,-3);
    let (x,y,z) = tup;
    println!("The value of y is {0} x is {1} and z is {2}",y,x,z);
    println!("Tuple: {:?}",tup);

    //arrays
    let numbers=[1,2,3,4,5];
    println!("Numbers: {:?}",numbers);
    let months=["Jan", "Feb", "Mar","Apr", "May","Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    let first_month=months[0];
    println!("First month: {}",first_month);
    println!("Months: {:?}",months);
}