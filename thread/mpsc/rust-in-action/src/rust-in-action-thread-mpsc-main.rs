#![allow(dead_code, unused_variables)]



/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-thread-mpsc_bin --bin rust-in-action-thread-mpsc-main```
///
/// ```cargo doc  --package rust-in-action-thread-mpsc_bin--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-thread-mpsc_bin```
///
/// ## What
/// `sleeping_subthread`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `100`
///
/// ## Example
/// In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.
///
/// ```compile_fail,ignore

use std::{thread, time};

 fn main() {
   let start = time::Instant::now();

   let handler_1 = thread::spawn(move || {
     let pause = time::Duration::from_millis(300);
       thread::sleep(pause.clone());
   });

   let handler_2 = thread::spawn(move || {
     let pause = time::Duration::from_millis(300);
       thread::sleep(pause.clone());
   });
   let handler_3 = thread::spawn(move || {
    let pause = time::Duration::from_millis(300);
      thread::sleep(pause.clone());
  });
   handler_1.join().unwrap();
   handler_2.join().unwrap();
   handler_3.join().unwrap();

   let finish = time::Instant::now();

   println!("{:?}", finish.duration_since(start));
 }
