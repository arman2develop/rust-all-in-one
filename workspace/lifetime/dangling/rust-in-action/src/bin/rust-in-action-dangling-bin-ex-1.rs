#![allow(dead_code, unused_variables)]
///
/// rust-in-action-dangling-bin-ex-1
///
/// # Commands
///
/// ```cargo run -q -p rust-in-action-dangle_bin --bin rust-in-action-dangling-bin-ex-1```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the integer number to verb the struc-name
///
/// # Return
/// `nothing`
///
/// ## Example
///
/// ```rust,compile_fail,no_run
///fn main() {
///let reference_to_nothing = dangle();
///}
///
///fn dangle() -> &String {
///    let s = String::from("hello");
///
///    &s
///}
/// ```
/// ```rust,compile_fail
/// let x = 5;
/// x += 2;
/// ```
fn main() {

 println!("Printed:{:?}","Hi");

}
