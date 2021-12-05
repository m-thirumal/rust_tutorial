fn main() {
    let name = "Thirumal";
    let number1:u32 = 12; //Immutable by default
    let number2:u32 = 1;
    let sum = number1 + number2;
    println!("Hello, world! {} --> the sum is: {}", name, sum);
    //Mutable
    let mut mutable_ariable = "Value is 1"; //Mutable variable
    println!("The mutable value {}", mutable_ariable);
    mutable_ariable = "Value is 2";
    println!("The mutable value {}", mutable_ariable);
    //Conditional statement
    let num = 10;
    if 2 > num {
        println!("2  >  {}", num);
    } else if  10 == num {
        println!("num is 10");
    } else {
        println!("Else condition -> num -> {}", num);
    }
    //
    let num1 = if num > 1 {3} else {5};
    println!("N {}", num1);
}
