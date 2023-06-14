use std::fmt;
use std::fmt::Formatter;
use std::path::Component;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{} + {}i", self.x, self.y)
    }
}

impl fmt::Debug for Point2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Complex {{ real:{real}, imag: {imag}}}", real = self.x, imag = self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D {
        x: 3.3,
        y: 7.2,
    };
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
}