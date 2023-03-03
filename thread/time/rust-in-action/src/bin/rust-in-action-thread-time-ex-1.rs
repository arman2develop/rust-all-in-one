#![allow(dead_code, unused_variables)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-thread-time_bin --bin rust-in-action-thread-time-ex-1```
///
/// ```cargo doc  --package rust-in-action-thread-time_bin--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-thread-time_bin```
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
///
/// //```compile_fail,ignore


// #![feature(thread_id)]

// use std::time;
// use std::thread;

// fn child_main() {
//    let thread_id = thread::current().id() as u64;
//    let delay_ms = 100 - (10 * thread_id);
//    let delay = time::Duration::from_millis(delay_ms);
//    std::thread::sleep(delay);

//    println!("hello from {:?}", thread_id);
// }

fn main() {
    // let mut child_threads = vec![];

    // for _ in ..5 {
    //     let child_thread = thread::spawn(child_main);
    //     child_threads.push(child_thread);
    // }

    // for child_thread in child_threads {
    //     child_thread.join();
    // }
    print!("done");
}
