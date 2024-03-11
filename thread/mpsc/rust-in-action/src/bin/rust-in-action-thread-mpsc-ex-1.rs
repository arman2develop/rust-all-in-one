#![allow(dead_code, unused_variables)]



/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-thread-mpsc_bin --bin rust-in-action-thread-mpsc-ex-1```
///
/// ```cargo doc  --package rust-in-action-thread-mpsc_bin--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-thread-mpsc_bin```
///
/// ## What
/// `sleep_strategy`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothig`
///
/// ## Example
/// In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.
///
/// ```compile_fail,ignore
use std::{thread, time};

fn main() {
  for n in 1..1001 {
    let mut handlers: Vec<thread::JoinHandle<()>> = Vec::with_capacity(n);

    let start = time::Instant::now();
    for _m in 0..n {
      let handle = thread::spawn(|| {
        let pause = time::Duration::from_millis(20);
        thread::sleep(pause);
      });
      handlers.push(handle);
    }
 /*
 A for loop does not permit modifications to the data being iterated over. Instead, the while loop allows us to repeatedly gain mutable access when calling handlers.pop().
 19 for handle in handlers {
20   handle.join();
21 }
 */
      // while let Some(handle) = handlers.pop() {
      //     handle.join();
      // }

    let finish = time::Instant::now();
    println!("{}\t{:02?}", n, finish.duration_since(start));
  }
}
