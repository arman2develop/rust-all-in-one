#![allow(dead_code, unused_variables)]
/// brain-teaser-educative-ex-2
///
/// # Commands
///
/// ```cargo run -q -p brain-teaser-educative_bin --bin brain-teaser-educative-ex-2```
///
/// ```cargo doc  --package brain-teaser-educative_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package brain-teaser-educative_bin```
///
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// ## Solution
/// ```rust,no_run,compile_fail,ignore
/// input.trim()
/// ```
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `Incorrect!`
///
/// ## Example
/// //```rust,no_run,compile_fail```

use std::io::stdin;

fn main() {
    println!("What is 3+2 ? Type your answer and press enter.");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Unable to read standard input");

    if input == "5" {
        println!("Correct!");
    } else {
        println!("Incorrect!");
    }
}
