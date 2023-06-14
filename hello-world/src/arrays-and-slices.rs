use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    let ys: [i32; 500] = [0; 500];

    // Inexing strat at 0.
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // `len` return the count of element in the array.
    println!("Number of elements in array: {}", xs.len());

    // Array are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // Slices can point to a section of an array.
    // They are of the form [starting_index..ending_index].
    // `starting_index` is the first position in the slice.
    // `ending_index` is one more than the last position in the slice.
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1..4]);

    // Example of empty slice `&[]`;
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array,&[]);
    assert_eq!(&empty_array,&[][..]); // Same but more verbose

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{}: {}",i,xval),
            None => println!("Slow down! {} is too far!",i),
        }
    }
}