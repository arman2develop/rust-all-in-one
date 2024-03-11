#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p other-math-max_bin --bin other-math-max-main```
///
/// ## What
/// `loop int`
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
///```rust

/// Find maximum within an array
fn maximum(_numbers: &[i64]) -> i64 {
   //let arr:[i64;5] = [1,2,23,4,5];
   let mut large:i64 = 0;
   let mut i:usize = 0;

   large=_numbers[0];
   while i<_numbers.len()
   {
		if large < _numbers[i] {
			large = _numbers[i]
		}
		i = i + 1;
   }
  large
}

fn main(){
  println!("{:?}",maximum(&[5,10,1,4,90]));
}
