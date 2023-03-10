#![allow(dead_code, unused_variables)]


/// rust-in-action-file-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-file_bin --bin rust-in-action-file-ex-3```
///
/// ```cargo doc  --package rust-in-action-file_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-file_bin```
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
/// // ```compile_fail,ignore

#[derive(Debug)] // <2> This enables `File` to work with `println!` and its `fmt!` sibling macros, used at the bottom of the code listing
struct File {
  name: String,
  data: Vec<u8>,
}

fn open(f: &mut File) -> bool { // <3> These two functions will remain inert for now
  true
}

fn close(f: &mut File) -> bool { // <3>
  true
}

fn read(f: &File, save_to: &mut Vec<u8>) -> usize { // <4> Return the "number of bytes read"
  let mut tmp = f.data.clone(); // <5> Make a copy of the data here, as `save_to.append()` will shrink the input vec
  let read_length = tmp.len();
  save_to.reserve(read_length); // <6> Not strictly necessary, but useful to know about
  save_to.append(&mut tmp); // <7> Allocate sufficient data in the `save_to` buffer to hold the contents of `f`
  read_length
}

fn main() {
  let mut f2 = File {
    name: String::from("2.txt"),
    data: vec![114, 117, 115, 116, 33],
  };

  let mut buffer: Vec<u8> = vec![];

  open(&mut f2);                          // <8> Do the hard work of interacting the the file.
  let f2_length = read(&f2, &mut buffer); // <8>
  close(&mut f2);                         // <8>

  let text = String::from_utf8_lossy(&buffer); // <9> Convert `Vec<u8>` to `String`. Any bytes that are not valid UTF-8 are replaced with �

  println!("{:?}", f2);
  println!("{} is {} bytes long", &f2.name, f2_length);
  println!("{}", text) // <10>  View [114, 117, 115, 116, 33] as an actual word
}
