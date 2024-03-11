#![allow(dead_code, unused_variables)]
///
/// brain-teaser-educative-ex-13
///
/// # Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p brain-teaser-educative_bin --bin brain-teaser-educative-ex-13```
///
/// ```cargo doc  --package brain-teaser-educative_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package brain-teaser-educative_bin```
///
/// ```cargo clippy --package brain-teaser-educative_bin --bin brain-teaser-educative-ex-13```
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

fn display_neutron_flow(polarity: isize) {
    println!(
        "Neutron Flow is {}",
        if polarity < 0 { "reversed"} else { "normal" }
    );
}

fn main() {
    let polarity = 1;
    {
        let polarity = polarity - 2;
        display_neutron_flow(polarity);
    }
    display_neutron_flow(polarity);
}
