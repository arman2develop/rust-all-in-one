#![allow(dead_code, unused_variables)]

/// rust-in-action-complex-bin-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-types-complex_bin --bin rust-in-action-complex-bin-ex-1```
///
/// ## What
///  make use of the is_nan() and is_finite() methods.
/// Inducing a crash, rather than silently proceeding with a mathematical error, allows you to debug close to what has caused the problem.
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [verb] the [your struct/func name]
///
/// # Return
/// `assert:true`
///
/// ## Example
/// //```rust,compile_fail,ignore

fn main() {
    let x: f32 = 1.0 / 0.0;
    assert!(x.is_infinite());
}
