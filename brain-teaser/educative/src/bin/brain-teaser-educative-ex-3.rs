#![allow(dead_code, unused_variables)]
/// brain-teaser-educative-ex-3
///
/// # Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p brain-teaser-educative_bin --bin brain-teaser-educative-ex-3```
///
/// ```cargo doc  --package brain-teaser-educative_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package brain-teaser-educative_bin```
///
///```cargo clippy --package brain-teaser-educative_bin --bin brain-teaser-educative-ex-3```
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// ## Solution
/// ```rust,no_run,compile_fail,ignore
/// there is not solution for conversain u32 and u64 but u64::from is stronger convertor than try_into and then into
/// ```
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `x does not equal y.`
///
/// ## Example
/// //```rust,no_run,compile_fail```

#[warn(clippy::pedantic)]
//#[warn(clippy::cast_possible_truncation)]
fn main() {
  let x : u64 = 4_294_967_296;
  let y = x as u32;
//  if x == (y as u64).into(){
//  if x == (y as u64).try_into().unwrap(){
  if x ==  u64::from(y){
    println!("x equals y.");
  } else {
    println!("x does not equal y."); }
}
