#![allow(dead_code, unused_variables)]
///
/// brain-teaser-educative-ex-14
///
/// # Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p brain-teaser-educative_bin --bin brain-teaser-educative-ex-14```
///
/// ```cargo doc  --package brain-teaser-educative_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package brain-teaser-educative_bin```
///
/// ```cargo clippy --package brain-teaser-educative_bin --bin brain-teaser-educative-ex-14```
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

use std::mem::size_of;

struct VeryImportantMessage {
    _message_type : u8,
    _destination : u16
}

fn main() {
    println!(
        "VeryImportantMessage occupies {} bytes.",
        size_of::<VeryImportantMessage>()
    );
}
