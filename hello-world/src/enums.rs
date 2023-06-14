use WebEvent::*;


enum WebEvent {
    // An `enum` variant may either be `unit-like`
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

// A function which take a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        PageLoad => println!("page loaded"),
        PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant,
        KeyPress(c) => println!("pressed '{}.", c),
        Paste(s) => println!("pasted \"{}\".", s),
        // Destrucutre `Click` finto `x` and `y`.
        Click { x, y } => {
            println!("clicked at x = {}, y= {}.", x, y);
        }
    }
}

fn main() {
   let pressed = KeyPress('x');
    // `to_owned()` create an owned  `String` from a string slice.
    let pasted = Paste("my text".to_owned());
    let click = Click {x: 20, y:80};
    let load = PageLoad;
    let unload = PageUnload;
    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);


}