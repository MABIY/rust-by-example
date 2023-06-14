use std::fmt;
use std::fmt::{Formatter, write};

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

fn transpose(pair: Matrix)-> Matrix {
    let Matrix(a,b,c,d) =pair;

    Matrix(a,c,b,d)

}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"( {} {} )\n\
                  ( {} {} )",self.0,self.1,self.2,self.3)
    }
}

fn main() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true
    );

    println!("Long tuple first vlaue: {}", long_tuple.0);
    println!("Long tuple second vlaue: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    println!("tuple of tuples: {:?}", tuple_of_tuples);


    // but long Tuples (more than 12 elements) cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("Too long tuple: {:?}", too_long_tuple);

    let pair = (1, true);
    println!("Pari is {:?}", pair);

    println!("The reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses.
    println!("One element tuple: {:?}", (5u32, ));
    println!("Just an integer: {:?}", (5u32));

    // Tuple can be destructured to create bindings.
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}",a,b,c,d);

    let matrix = Matrix(1.1,1.2,2.1,2.2);

    println!("Matrix:\n{}",matrix);
    println!("Transpose:\n{}",transpose(matrix));

}