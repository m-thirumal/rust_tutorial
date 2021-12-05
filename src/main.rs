use std::io;
mod ownership;
mod data_structure;
mod exception_handle;

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
    // function
    let z = add(2, 5);
    println!("Result {}", z);
    //Structure
    let temp = Template {
        x: 12,
        y: -43,
        flag: true
    };
    println!("The temp values are x {} y {} flag {}", temp.x, temp.y, temp.flag);
    // SOME & NONE
    some_none_example();
    //Read input from user
    //read_user_input();
    //Ownership
    ownership::ownership_example();
    // Data structure
    data_structure::array_example();

    data_structure::tuple_example();

    data_structure::vector_example();

    data_structure::struct_example();

    data_structure::enum_example();

    data_structure::hashmap_example();
    //Exception handling
    exception_handle::basic_example();

    exception_handle::handle_error();

}

fn some_none_example() { //Avoid NULL
    println!();
    let some_number = Some(12);
    println!("The some value {}", some_number.unwrap());
    let none_number: Option<u32> = None;
    println!("The none value {}", none_number.is_none());
    println!();
}

fn read_user_input() {
    println!();
    println!("Please enter number");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input");
    
    let string_to_number: u32 = user_input.trim().parse().expect("Not able to convert to number");

    println!("Number from input is {}", string_to_number);
    println!();
}

fn add(x:u32, y:u32) -> u32 {
    println!("Additinal function ");
    x + y
}

struct Template {
    x:u32,
    y:i32,
    flag:bool
}

