use std::fs::File;

pub fn basic_example(){
    panic!("---------------I am panic from expection handle example---------");
}

pub fn handle_error() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file {:?}", error)
    };
    //Expect
    let f1 = File::open("hello.txt").expect("Failed to open file");
    let f2 = File::open("hello.txt").unwrap();
    
}