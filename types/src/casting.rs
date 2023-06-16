#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // Error! No implicit conversion
    // let integer: u8 = decimal;
    // FIXME ^ Comment out this line

    // Explicit conversion
    let integer = decimal as u8;
    println!("{integer}");

    let character  = integer as char;
    println!("{character}");

    // Error! there are limitations in conversion rules
    // A floagt cannot be directly converted to a char.
    // let character = decimal as char;
    // FiXME ^ Comment out this line

    println!("Casting: {} -> {} -> {}",decimal,integer,character);

    // when casting any value to an unsigned type ,T,
    // T::MAX + 1 is added or subtracted until the value
    // fits into the new type

    // 1000 already fits in 1 u16
    println!("1000 as a u16 is: {}",1000 as u16);

    // 1000 - 256 - 256 - 265 = 232
    // under the hood, the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bit (MSB) get truncated
    println!("1000 as a u8 is: {}",1000 as u8);

    // -1 + 256 = 25u
    println!(" -1 as a u8 is : {}", (-1i8) as u8);

    // for positive numbers, this is same as the modulus
    println!("1000 mod 256 is : {}",1000 %256);

    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type. if the most significatn
    // bit of the value is 1, then the value is negative.

    // unless it already fit, of course.
    println!(" 128 as a i16 is: {}", 128 as i16);

    // In boundary case 128 value in 8-bit two's complement
    println!("128 as a i8 is : {}", 128 as i8);

    //repeating the example above
    println!("1000 as a u8 is : {}", 1000 as u8);
    // and the value of 232 in 8-bit two's complement representation is -24
    println!(" 232 as a i8 is : {}", 232 as i8);

    // Since Rust 1.45, the `as` keyword performs a *saturating cast*
    // when casting from float to int . if the floating point value exceeds
    // the upper bound or is less than the lower bound , the returned value
    // will be equal to the bound crossed
    // 300.0 as u8 is 255
    println!(" 300.0 as u8 is: {}", 300.0_f64 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is: {}",-100.0_f64 as u8);
    // nan as u8 is 0
    println!(" nan as u8 is : {}",f64::NAN as u8);

    unsafe  {
        // 300.0 as u8 is 44
        println!(" 300.0 as u8 is : {}",300.0_f64.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is : {}", (-100.0_f64).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!(" nan as u8 is : {}",f64::NAN.to_int_unchecked::<u8>());

    }
}
