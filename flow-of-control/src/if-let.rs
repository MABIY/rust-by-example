// Our example enum
enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn main() {
    let optional = Some(7);

    match optional {
        Some(i) => {
            println!("This is really long string and `{:?}`", i);
            // Needed 2 indentations  just so we could destructure
            // `i` from the option.
        }
        _ => {},
        // Required because `match` is exhaustive it seem
        // like wated space?
    }

    // All have type `Optiaon<i32>`
    let number = Some(7);
    let letter: Option<i32> =None;
    let emoticon: Option<i32> =None;

    // The `if let` construct reads: "f `let` destructures `number` into
    // `Some(i)`, evaluate the block ('{}')"
    if let Some(i) = number {
        println!("Matched {:?}",i);
    }

    // if you need to specify a failure case.
    if let Some(i)  = letter {
        println!("Matched {:?}",i);
    } else {
        // Destructure failed. Change to the failure case.
        println!("Didn't match a number.Let's go with a letter!");
    }

    // Provide an altered failing condition

    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}",i);
       // Destructure failed, Evaluate an `else if`  condition to see if the
       // alternate failure branch should be token
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // The condition evaluated false, This branch is the default:
        println!("I don't like letters. Let's go with an emoticon:)!");
    }

    // Create example variables
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // Variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // Variable b does not match Foo::Bar
    // So this will print nothing
    if let Foo::Bar =b {
        println!("b is foobar");
    }

    // Variable c matches Foo::Qux which has a value
    // Similar to Some() in the previous example
    if let Foo::Qux(value) = c {
        println!("c is {}",value);
    }

    // Binding also works with `if let`
    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }


}

