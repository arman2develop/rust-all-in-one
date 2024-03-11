#![allow(dead_code, unused_variables)]
/// brain-teaser-educative-ex-5
///
/// # Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p brain-teaser-educative_bin --bin brain-teaser-educative-ex-5```
///
/// ```cargo doc  --package brain-teaser-educative_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package brain-teaser-educative_bin```
///
/// ```cargo clippy --package brain-teaser-educative_bin --bin brain-teaser-educative-ex-5```
///
/// ## What
/// `TODO`
///
/// ## How
/// `Cause use of ASCII code will have 1 more len  additional char `
/// > Note: The Char variables don’t represent a single ASCII character. Instead, they represent a Unicode scalar value. The scalar value can represent a single glyph or a modification to another glyph.
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
/// `Halló heimur is 13 characters long.`

///
/// ## Example
/// //```rust,no_run,compile_fail /// ```
/// 
//const HELLO_WORLD : &'static str = "Halló heimur";// result 13
const HELLO_WORLD : &'static str = "Привет мир";//result 19
//const HELLO_WORLD : &'static str = "❤";//3
fn main() {
    println!("{} is {} characters long. nth 4 is {}",
        HELLO_WORLD,
        HELLO_WORLD.len(),
        HELLO_WORLD.chars().nth(4).unwrap()
    );
}
