#![allow(dead_code, unused_variables)]



/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-thread-sharedstate_bin --bin rust-doc-thread-sharedstate-ex-2```
///
/// ```cargo doc  --package rust-doc-thread-sharedstate_bin--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-thread-sharedstate_bin```
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
/// `nothig`
///
/// ## Example
/// In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.
///
/// ```compile_fail,ignore
use std::sync::Mutex;
use std::thread;
use std::rc::Rc;

fn main() {
    let counter = Mutex::new(0);
    //let mut handles = vec![];

    // for _ in 0..10 {
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();

    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("Result: {}", *counter.lock().unwrap());
}
/* Posibility multithread by rc:  Attempting to use Rc<T> to allow multiple threads to own the Mutex<T>

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

*/
