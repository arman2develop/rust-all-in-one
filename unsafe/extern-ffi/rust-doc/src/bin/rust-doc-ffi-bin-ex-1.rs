#![allow(dead_code, unused_variables)]

/// rust-doc-ffi-main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-ffi_bin --bin rust-doc-ffi-main```
///
/// ## What
/// `TODO`
///
/// ## How
/// `This usage of extern does not require unsafe.`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// //```rust,compile_fail,ignore

fn main() {
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
}
