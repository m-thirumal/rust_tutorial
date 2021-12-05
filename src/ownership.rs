/*
* ----Rules of Ownership-----------
* 1. Each value in Rust has a varaiable that's called it's owner
* 2. There can only be one owner at a time
* 3. When the owner goes out of scope, the value will be dropped
*/
pub fn ownership_example() {
    let name = String::from("Thirumal");
    // We are passing (&name) the reference in the method argument, because we need to use `name` in the print
    let length = calculate_length(&name); 
    println!("The name {} length is {}", name, length);

    // Modify the method argument and return it.
    let mut s = String::from("Hello");
    modify_method_argument(&mut s);
    println!("The modified method argument value is {}", s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn modify_method_argument(s: &mut String) {
    s.push_str(" World")
}