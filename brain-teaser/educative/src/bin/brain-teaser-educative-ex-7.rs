#![allow(dead_code, unused_variables)]
///
/// brain-teaser-educative-ex-7
///
/// # Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p brain-teaser-educative_bin --bin brain-teaser-educative-ex-7```
///
/// ```cargo doc  --package brain-teaser-educative_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package brain-teaser-educative_bin```
///
/// ```cargo clippy --package brain-teaser-educative_bin --bin brain-teaser-educative-ex-7```
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
use std::f32::consts::PI;

pub struct Degrees(pub f32);
pub struct Radians(pub f32);

impl Degrees {
    pub fn new(angle: f32) -> Self {
        Self(angle)
    }
}

impl From<Degrees> for Radians {
    fn from(item : Degrees) -> Self {
        Self(item.0 * PI / 180.0)
    }
}

fn main() {
    let one_eighty_degrees = Degrees::new(180.0);
    let one_eighty_radians : Radians = one_eighty_degrees.into();
    println!("180 Degrees in Radians = {}", one_eighty_radians.0);
}
