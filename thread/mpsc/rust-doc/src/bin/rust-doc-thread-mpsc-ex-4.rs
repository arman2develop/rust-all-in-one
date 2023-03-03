#![allow(dead_code, unused_variables)]



/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-thread-mpsc_bin --bin rust-doc-thread-mpsc-ex-4```
///
/// ```cargo doc  --package rust-doc-thread-mpsc_bin--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-thread-mpsc_bin```
///
/// ## What
/// `thread-channel`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothig`
///
/// ## Example
/// In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.
///
/// ```compile_fail,ignore

use std::thread;

fn main() {
    unimplemented!();
   // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    //     println!("val is {}", val);
    // });

    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);
}
