fn main() {
    // variable binding
    let x = 5;

    // expression;
    x;
    x + 1;
    15;

    let x = 5u32;

    {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let y =  {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };
    let z = {
        2 * x;
    };

    println!("x is {:?}",x);
    println!("y is {:?}",y);
    println!("z is {:?}",z);

}