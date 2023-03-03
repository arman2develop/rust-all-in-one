#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]



/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-ffi_bin --bin rust-doc-ffi-main```
///
/// ```cargo doc  --package rust-doc-ffi_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-ffi_bin```
///
/// ## What
// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `Announcement! Today is someone's birthday!`
/// `The longest string is abcd`
///
/// ## Example
/// //```rust,compile_fail,ignore
///
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
