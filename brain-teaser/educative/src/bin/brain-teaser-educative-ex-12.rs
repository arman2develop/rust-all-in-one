#![allow(dead_code, unused_variables)]
///
/// brain-teaser-educative-ex-12
///
/// # Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p brain-teaser-educative_bin --bin brain-teaser-educative-ex-12```
///
/// ```cargo doc  --package brain-teaser-educative_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package brain-teaser-educative_bin```
///
/// ```cargo clippy --package brain-teaser-educative_bin --bin brain-teaser-educative-ex-12```
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
/// `The program loop runs forever since there’s no break keyword to stop it.`
///
/// ## Example
/// //```rust,no_run,compile_fail /// ```
///

fn main() {
    loop {
        let buffer = (0..1000).collect::<Vec<u32>>();
        std::mem::forget(buffer);
        print!(".");
    }
}
