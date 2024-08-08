
pub fn run(){
    println!("Running");
    another_function(5,6.2);
    greater_function();
    answer();
    println!("You are the goat of this shit!");

}

fn another_function(x:i32,y:f64){
    println!("Another function");
    println!("The value returned is {0} and {1}",x,y);
}

fn greater_function(){
    let x=5;
    let y = {
        let x=3;
        x+1
    };
    println!("if x is {1} then y returns {0}",y,x);
}

//fnctions with return values
fn five() -> i32 {
    5
}
fn answer(){
    let x=five();
    let z=plus_five(5);
    println!("The value returned is {0}", x);
    println!("we get {}", z);
}

fn plus_five(x:i32) -> i32 {
    x+1

}