#![allow(dead_code, unused_variables)]
///
/// brain-teaser-educative-ex-17
///
/// # Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p brain-teaser-educative_bin --bin brain-teaser-educative-ex-17```
///
/// ```cargo doc  --package brain-teaser-educative_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package brain-teaser-educative_bin```
///
/// ```cargo clippy --package brain-teaser-educative_bin --bin brain-teaser-educative-ex-17```
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
/// `nothing`
///
/// ## Example
/// //```rust,no_run,compile_fail /// ```
///

fn main() {
    let mut my_vec = Vec::with_capacity(1);
    my_vec.push("Hello");
    println!("{}", my_vec.capacity());

    my_vec.push("World");
    println!("{}", my_vec.capacity());
}
