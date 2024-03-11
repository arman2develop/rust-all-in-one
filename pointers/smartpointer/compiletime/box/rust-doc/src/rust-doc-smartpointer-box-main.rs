#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-smartpointer-box_bin --bin rust-doc-smartpointer-box-main```
///
/// ```cargo doc  --package rust-doc-smartpointer-box_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-smartpointer-box_bin```
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
///  `TODO`
///
/// //```rust,compile_fail,no_run,ignore
fn main() {
    let x = 5;
    let y = Box::new(x);// Box::new means '&'
    let y = Box::new(String::from("Arman"));
    let yy = String::from("Arman");

    let z= &(*y)[..];
    let zz= &(*yy)[..];

    let o= &(y)[..];
    let oo= &(yy);
    println!("Printed:{:}",&oo);

    assert_eq!(5, x);
    assert_eq!("Arman", z);
    assert_eq!("Arman", zz);
    assert_eq!("Arman",o);
    assert_eq!("Arman",oo);
}
