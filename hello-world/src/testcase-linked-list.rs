use std::mem;
use crate::List::Cons;

#[derive(PartialEq)]
enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Method can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        List::Nil
    }

    // consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        Cons(elem, Box::from(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to b matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List` and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference  `&T`
        // after Rust 2018 you can use self here and tail (with no ref)  below as well,
        // rust will infer &s and ref tail.
        match self {
            // Can't take ownership of the tail, because `self` is borrowed
            // instead take a reference to the tail
            Cons(_, tail) => 1 + tail.len(),
            List::Nil => 0
        }
    }

    // Return representation of the list as as (heap allocated ) string
    fn stringify(&self) -> String {
        match self {
            // `format!` is similar to  `print!`, but return a heap
            // allocated string instead of printing to the console
            Cons(head, tail) => {
                format!("{}, {}", head, tail.stringify())
            }

            List::Nil => format!("Nil")
        }
    }
}

fn main() {
    // Create an empty linked list;
    let mut list = List::new();

    //Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // show the final state of the list
    println!("linked list has length: {}",list.len());
    println!("{}",list.stringify());
}