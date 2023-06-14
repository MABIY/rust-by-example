use std::io;

type NanoSecond = u64;
type Inch = u64;
type U64 = u64;


fn main() {
    // `NanoSecond` = `Inch` = `U64` =u64
    let nanoseconds: NanoSecond = 5;
    let inches: Inch = 2;

    //Note that type aliases *don't* provide any extra type safely, because
    // aliases aer *not* new types
    println!("{} nanoseconds + {} inches = {} unit?", nanoseconds, inches, nanoseconds + inches)
}