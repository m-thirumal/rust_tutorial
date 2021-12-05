use std::{collections::HashMap, ops::Add};

pub fn array_example() {
    let integer_array = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    print!("Integer array -->");
    for i in &integer_array {
        print!(" {}", i);
    }
    println!();
    print!("String array -->");
    let months = ["Jan", "Feb", "Mar", "Apr"];
    for i in &months {
        print!(" {}", i);
    }
    println!();
    //Defing data type and length
    print!("Data defined array -->");
    let num_list:[u32; 5] = [1, 2, 3, 4, 5];
    for i in &num_list {
        print!("{}", i);
    }
    println!();
    //Repeat the value n times
    print!("Repeatable array -->");
    let repeat_values = ["hi "; 10];
    for i in &repeat_values {
        print!(" {}", i);
    }
    println!();
    println!("The size of repeatable array is {}", repeat_values.iter().count());    
    println!();
}

pub fn tuple_example() {
    let t = (1, 'S', 2.4, "Thirumal");
    println!("Tuple starting element --> {}", t.0);
    //
    let t1:(u32, f32) = (3, 3.4);
    println!("2nd Tuple (t1) element is --> {} ", t1.1);
}

//Vector uses heap instead of stack everything else are same as array. Iteratable
pub fn vector_example() {
    // syntax -> vec![]
    let mut vector = vec![1, 2, 3];
    vector.push(11);
    println!("The values of vector are {:?}", vector);


}

struct Rectangle { //public struct is also possible
    width: f32,
    length: f32
}

impl Rectangle {

    fn area(&self) -> f32 {
        self.width * self.length
    }

    fn perimeter(&self) -> f32 {
        2.0 * (self.width + self.length)
    }
    
}

pub fn struct_example() {
    let rectangle = Rectangle {
        width:12.3,
        length:3.0
    };
    println!("The area of rectangle {}", rectangle.area());

    println!("The perimeter of rectangle {}", rectangle.perimeter());
}

pub fn enum_example()  {
    #[derive(Debug)]
    enum Burger {
        SMALL,
        MEDIUM,
        LARGE,
        EXTRALARGE
    }
    println!("The Burger size {}", Burger::MEDIUM as u8);

    //let burger_size = Burger::EXTRALARGE;
    //let burger_size = Burger::LARGE;
    let burger_size = Burger::SMALL;

    match burger_size { //This is similar to switch
        Burger::SMALL => {
            println!("The Burger size is SMALL");
        }

        Burger::MEDIUM => {
            println!("The Burger size is MEDIUM");
        }

        Burger::LARGE => {
            println!("The Burger size is LARGE");
        }
        Burger::EXTRALARGE => {
            println!("The Burger size is EXTRALARGE");
        }
    }
}

pub fn hashmap_example() {
    println!();
    let mut h = HashMap::new();
    
    let key1 = String::from("1");
    h.insert(&key1, "Thirumal");
    println!("The value of 1 is {:?}", h.get(&key1));
    let key2 = String::from("2");
    println!("The value of 1 is {:?}", h.get(&key2));
    println!();
}