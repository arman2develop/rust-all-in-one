#![allow(dead_code, unused_variables)]
/// rust-doc-closure-ex-4
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-closure_bin --bin rust-doc-closure-ex-4```
///
/// ```cargo doc  --package rust-doc-closure_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-closure_bin ```
///
/// ## What
/// `TODO`
///
/// ## How
/// `Closures are represented by traits, which means you can’t return closures directly. In most cases where you might want to return a trait, you can instead use the concrete type that implements the trait as the return value of the function. But you can’t do that with closures because they don’t have a concrete type that is returnable; you’re not allowed to use the function pointer fn as a return type`
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
/*
The closure then has ownership of x, and main isn’t allowed to use x anymore in the println! statement. Removing println! will fix this example.
*/
// fn main() {
//     let x = vec![1, 2, 3];

//     let equal_to_x = move |z| z == x;

//     println!("can't use x here: {:?}", x);

//     let y = vec![1, 2, 3];

//     assert!(equal_to_x(y));
// }


// fn main() {
//     let x = 4;

//     fn equal_to_x(z: i32) -> bool {
//         z == x
//     }

//     let y = 4;

//     assert!(equal_to_x(y));
// }


// fn main() {
//     let x = 4;

//     let equal_to_x = |z| z == x;
//        println!("can use x here: {:?}", x);
//     let y = 4;

//     assert!(equal_to_x(y));
// }
fn main() {
    unimplemented!();
}
