static NUM: i32 = 18;

// return a reference to  `NUM` where its `'static`
// lifetime is coerced to that of the input argument.
fn coerce_static(_: &i32) -> &i32 {
    &NUM
}

fn main() {
    {
        // Make a 'string' literal and print it:
        let static_string = "I'm in read-only memory";
        println!("static_string: {static_string}");

        //when static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
    }

    {
        // Make an integer to use for `coerce--static`:
        let lifetime_num = 9;

        // coerce `NUM` to lifetime of `lifetime_num`:
        let coerced__static = coerce_static(&lifetime_num);
    }

    println!("NUM: {} stays accessible!",NUM);
}