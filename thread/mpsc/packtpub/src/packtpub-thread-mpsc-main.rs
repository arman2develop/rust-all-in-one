#![allow(dead_code, unused_variables)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p packtpub-thread-mpsc_bin --bin packtpub-thread-mpsc-main```
///
/// ```cargo doc  --package packtpub-thread-mpsc_bin--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package packtpub-thread-mpsc_bin```
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
/// `100`
///
/// ## Example
/// In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.
///
/// //```compile_fail,ignore


// use std::thread::spawn;
// pub use std::sync::mutex;



fn main() {
    unimplemented!();
    // let (cs,cr) = mutex::channel::<i32>();
    // let h = spawn(move||{
    //     loop {
    //         match cr.recv() {
    //             Ok(v) => {
    //                 println!("Value {}",v);
    //             },
    //             Err(e) =>{
    //                 println!("Err = {:?}",e);
    //                 return;
    //             }
    //         }
    //     }
    // });

    // let cs2 = cs.clone();

    // spawn(move|| {
    //     for j in 10 ..20 {
    //         cs2.send(j).unwrap();
    //     }
    // });

    // for i in 0..10{
    //     cs.send(i).expect("Reciever dropped early");
    // }

    // drop(cs);

    // h.join();
}
