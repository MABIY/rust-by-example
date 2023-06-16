fn main() {
    // Try changing the value in the array, or make it a slice!
    let array = &[3, -2, 6][..];

    match array {
        // Bind the second and the third element to the respective variables
        [0, second, third] =>
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),

        // Single values can be ignored with _
        [1, _, third] => println!("array[0] = 1, array[2] = {} and array[1] was ignored", third),

        // You can also bind some and ignore the rest
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones where ignored",
            second
        ),

        // The code below would not compile
        // [-1,second] => ...,
        // or store them in another array/slice (the type depends on
        // that of the value that is being matched against)
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were{:?}",
            second, tail
        ),

        [first] => println!("test"),
        // Combining these patterns, we can , for example, bind the first and
        // last values, and store the rest of them in a single array
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
        [..] => println!(""),
    }
}
