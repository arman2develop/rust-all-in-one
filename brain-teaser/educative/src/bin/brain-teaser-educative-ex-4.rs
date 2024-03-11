#![allow(dead_code, unused_variables)]
/// brain-teaser-educative-ex-4
///
/// # Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p brain-teaser-educative_bin --bin brain-teaser-educative-ex-4```
///
/// ```cargo doc  --package brain-teaser-educative_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package brain-teaser-educative_bin```
///
///```cargo clippy --package brain-teaser-educative_bin --bin brain-teaser-educative-ex-4```
///
/// ## What
/// `TODO`
///
/// ## How
/// > For avoid Overflow:
/// >Rust’s numeric types implement the following series of checked functions:
/// checked_add(), checked_div(), checked_mul(), and checked_sub() along with few others.
///
/// ## Solution
/// Add lines to Cargo:
/// [profile.dev]
/// opt-level = 1 // 1 for minimal optimization and good debugging.
///
/// ```rust,no_run,compile_fail
///     use std::num::Wrapping;
///     let mut counter = Wrapping(0i8);
///     loop {
///         println!("{}", counter);
///         counter += Wrapping(1i8);
///     }
/// //Result 0..127- -128...0 , repeat
/// ```
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `print: 0...127 and then show rust_unwind status: thread 'main' panicked at 'attempt to add with overflow'`
///
/// ## Example
/// ```rust,no_run,compile_fail
/// if let Some(n) = x.checked_add(b) {
/// //It worked, n contains the result.
///} else {
/// //Overflow occurred - handle the error.
///}
/// ```
fn main() {
  let mut counter : i8 = 0;
  loop {
    println!("{}", counter);
      counter += 1;
  }
}
