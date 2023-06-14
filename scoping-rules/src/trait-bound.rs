use std::fmt::Debug;

fn print_it<T>(input: T)
    where T: Debug + 'static {
    println!("static value passed in is: {:?}", input);
}

fn main() {
    // i is owned and contains no referenced ,  thus it's 'static:
    let i = 5;
    print_it(i);

    //oops, &i only has the lifetime defined by the scope of
    // mina(), so it's not 'static
    // print_it(&i);
}