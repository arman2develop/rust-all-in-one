#![allow(dead_code, unused_variables)]

use std::rc::Rc;

/// rust-doc-trait-associate-type-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-trait-associate-type-and-trait_bin --bin  rust-doc-trait-associate-type-ex-1```
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
/// `unimplemented`
///
/// ## Example
/// //``rust,no_run,compile_fail,ignore

#[derive(Debug)]
struct GroundStation {}

fn main() {
  let base = Rc::new(GroundStation {});   // <2>

  println!("{:?}", base);                 // <3>
  println!("{:?}", base);                 // <3>
}
