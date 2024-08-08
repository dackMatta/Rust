pub fn run(){
    println!("Running");
   // loop_example();
    loop_through_collection();

}
//conditional loops

/*fn loop_example(){
    let mut number=3;
    while number<10 {
        println!("Number: {}", number);
        number+=1;
    }
    while number!=0 {
        println!(": {}", number);
        number=number-1;
    }
    println!("lift off!")
}
*/

//looping through a collection

fn loop_through_collection(){
    let a=[1,2,3,4,5];
    let mut index=0;
    while index<5{
        println!("the value is: {}", a[index]);
        index=index+1;
    }
   }
