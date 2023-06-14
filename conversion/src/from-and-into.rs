use std::fmt::{Debug};

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self {
            value,
        }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    // Try removing the type annotation
    let num: Number = int.into();
    println!("My number is {:?}", num);

    let string = "hello".to_string();
    let other_string = String::from("hello");

    assert_eq!(string,other_string);
}
