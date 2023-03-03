#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-linkedlist_bin --bin rust-doc-linkedlist-main```
///
/// ```cargo doc  --package rust-doc-linkedlist_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-linkedlist_bin ```
///
/// ## What
/// `TODO`
///
/// ## How
///  When we instantiate any linkedlist implementing value (any heap allocated type), the Rust compiler inserts linkedlist method calls after every end of scope, after compilation. So, we don't need to manually call linkedlist on these instances. This kind of automatic reclamation based on scope is inspired by the RAII principle of C++.
/// That changes a bit if we want to keep the structure this simple but still have a double-linked list, since then we actually have to change the existing structure.
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
///  `TODO`
///
/// //rust,compile_fail,no_run,ignore
///  `TODO`
use crate::List::*;

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

/// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        Cons(elem, Box::new(self))
    }

    /// Return the length of the list
    /// `self` has to be matched, because the behavior of this method
    /// depends on the variant of `self`
    /// `self` has type `&List`, and `*self` has type `List`, matching on a
    /// concrete type `T` is preferred over a match on a reference `&T`
    /// after Rust 2018 you can use self here and tail (with no ref) below as well,
    /// rust will infer &s and ref tail.
    /// See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
    fn len(&self) -> u32 {

        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            Cons( _,ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Nil => 0
        }
    }

    /// Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
