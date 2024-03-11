#![allow(dead_code, unused_variables)]
///
/// brain-teaser-educative-ex-15
///
/// # Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p brain-teaser-educative_bin --bin brain-teaser-educative-ex-15```
///
/// ```cargo doc  --package brain-teaser-educative_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package brain-teaser-educative_bin```
///
/// ```cargo clippy --package brain-teaser-educative_bin --bin brain-teaser-educative-ex-15```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// ## Solution
///
/// //```rust,no_run,compile_fail```
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `The program will display node 1, node 2, and then node 1 again. The output repeats itself until the program exits while displaying the following message: thread 'main' has overflowed its stack.`
///
/// ## Example
/// //```rust,no_run,compile_fail /// ```
///

use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

fn main() {
    let mut head = Some(Rc::new(
        RefCell::new(Node{ elem: 1, next: None })
    ));
    head
        .as_mut()
        .unwrap()
        .borrow_mut()
        .next = Some(Rc::new(RefCell::new(
            Node{ elem: 2, next: head.clone() })
        ));

    println!("{:?}", head);
}
