#![allow(dead_code, unused_variables)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p packtpub-file_bin --bin packtpub-file-main```
///
/// ```cargo doc  --package packtpub-file_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package packtpub-file_bin ```
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
/// ```rust,compile_fail,ignore


use std::fs::File;
use std::io::{Read,Write,copy};


fn main()->std::io::Result<()> {
    let mut s = String::new();

    let mut f = File::open("/mnt/home/rust-all-in-one/workspace/data/io/file/packtpub/src/data/from.md")?;
    f.read_to_string(&mut s)?;

    println!("{}",s);


    let mut t = File::create("data/to.md")?;

    //copy(&mut f,&mut t)?;

    t.write_all(&s.into_bytes())?;

    Ok(())
}
