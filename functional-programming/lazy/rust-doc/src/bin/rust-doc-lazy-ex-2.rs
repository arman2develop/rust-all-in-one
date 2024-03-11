#![allow(dead_code, unused_variables)]
/// rust-doc-lazy-ex-2
///
/// ## Commands
///
/// ```cargo test -q -p rust-doc-lazy_bin --bin rust-doc-lazy-ex-2```
///
/// ```cargo doc  --package rust-doc-lazy_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-lazy_bin ```
///
/// ## What
/// `GlobalState`
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
#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use std::sync::Mutex;
    lazy_static! {
        static ref MUTEX: Mutex<i32> = Mutex::new(0);
    }
    #[test]
    fn first_test() {
        let _guard = MUTEX.lock().expect("couldn't acquire lock");
        println!("first test is running");
    }
    #[test]
    fn second_test() {
        let _guard = MUTEX.lock().expect("couldn't acquire lock");
        println!("second test is running");
    }
}

fn main(){
    unimplemented!();
}
