#![allow(dead_code, unused_variables)]
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
/// rust-doc-error-handling-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-error-handling_bin --bin  rust-doc-error-handling-ex-1```
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
/// `unimplemented`
///
/// ## Example
/// //``rust,no_run,compile_fail,ignore
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("/mnt/home/rust-all-in-one/workspace/errorhandling/rust-doc/src/hello.txt")?.read_to_string(&mut s)?;//=  fs::read_to_string("hello.txt")
    /*
    Rust provides the convenient fs::read_to_string function that opens the file, creates a new String, reads the contents of the file, puts the contents into that String, and returns it. Of course, using fs::read_to_string doesn’t give us the opportunity to explain all the error handling, so we did it the longer way first.
    */
        Ok(s)
}

fn main() {
    read_username_from_file();
}
