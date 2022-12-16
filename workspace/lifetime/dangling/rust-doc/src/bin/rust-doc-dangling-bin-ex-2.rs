#![allow(dead_code, unused_variables)]

/// rust-doc-dangling-bin-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-dangle_bin --bin  rust-doc-dangling-bin-ex-2```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [verb] the [your struct/func name]
///
/// # Return
/// `unimplemented`
///
/// ## Example
/// ```rust,no_run,compile_fail,ignore
///```
///
fn main() {
    unimplemented!()
    // let r;//This code won’t compile because the value r is referring to has gone out of scope before we try to use it

    // {
    //     let x = 5;
    //     r = &x;//The variable x doesn’t “live long enough.” The reason is that x will be out of scope when the inner scope ends on line 7
    // }

    // println!("r: {}", r);
}
/* dangle
    {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+
*/

/* undangle
fn main() {
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
}

*/
