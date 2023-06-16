// `allow` required to silence warning because only
//  one variant is used.
#[derive(Debug)]
#[allow(dead_code)]
enum Color {
    // These   3 are specified solely by their name.
    Red,
    Blue,
    Green,
    // These likewise tie `u32` tuples to different names: color models
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32,u32),
}

fn main() {
    let color = Color::RGB(122, 17, 40);

    println!("What color is it?");
    // An `enum` can be destructured using a `match`.
    match color {
        n @ Color::Red => println!("The color is Red!{n:?}"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {r}, green: {g}, and blue: {b}!"),
        Color::HSV(h, s, v) =>
            println!("Hue: {h}, saturation: {s}, and value: {v}!"),
        Color::HSL(h, s, l) =>
            println!("Red: {h}, saturation: {s}, and lightness: {l}!"),
        Color::CMY(c, m, y) =>
            println!("Cyan: {c}, magenta: {m}, yellow: {y}!"),
        Color::CMYK(c, m, y,k) =>
            println!("Cyan: {c}, magenta: {m}, yellow: {y}, key (black): {k}!"),

        // Don't need another arm because all variants have been examined
    }
}