pub fn run(){
    println!("Running");
    run_statement();
    multiple_conditions();
    number_if();
}

fn run_statement(){
    println!("initiating the statement");
    let x=5;
    if x < 10{
        println!("x is less than 10");
    }else {
        println!("x is greater than 10");
    }
}

//multiple conditions
fn multiple_conditions(){
    let color="blue";
    let number=2;   
    if color == "blue" && number == 3{
        println!("Color is blue and number is 3");
    }else {
        println!("non of the above");
    }
}

//Using if in a let Statement
fn number_if(){
    let number=5;
    let result=if number < 10{
        "Small"
    } else if number < 20 {
        "Medium"
    } else {
        "Large"
    };
    println!("Number is {} and it is {}",number,result);
}