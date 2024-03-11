#![allow(dead_code, unused_variables)]
use std::ops::Add;
/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-trait-bound-operator_bin --bin rust-doc-trait-bound-operator-main```
///
/// ```cargo doc  --package rust-doc-trait-bound-operator_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-trait-bound-operator_bin ```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
///  `TODO`
///
///


struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
fn main(){
    unimplemented!();
}
