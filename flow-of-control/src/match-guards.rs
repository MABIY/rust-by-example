// #[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn main() {
    let temperature = Temperature::Celsius(35);
    // ^ TODO try different values for `temperature`

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{t} is above 30 Celsius"),
        //The `if condition` part ^ is a guard
        Temperature::Celsius(t) => println!("{t} is below 30 Celsius"),

        Temperature::Fahrenheit(t) if t > 86 => println!("{t}F is above 86 Fahrenheit"),
        Temperature::Fahrenheit(t) => println!("{t}F is below 86 Fahrenheit"),
    }

    let number: u8 = 4;
    match number {
        i if i==0 => println!("zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => unreachable!("should never happen."),
        // TODO ^ uncomment to fix compilation
    }
}