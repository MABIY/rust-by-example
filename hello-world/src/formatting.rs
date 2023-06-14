use std::fmt;
use std::fmt::{Formatter, write};

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "RGB")?;
        let Color {
            red, green
            ,
            blue,
        } = self;
        write!(f, "({red}, {green}, {blue})");
        write!(f, " {red:#02x}{:0>2}{:0>2}",format!("{:X}",green),format!("{:X}",blue))
    }
}

fn main() {
    let c:u8 = 255;
    println!("{c:0>2}");
    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ] {
        println!("{}", city);
    }

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ] {
        println!("{}", color);
    }
}