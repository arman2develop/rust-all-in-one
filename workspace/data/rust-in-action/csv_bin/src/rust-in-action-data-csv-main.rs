#![allow(dead_code, unused_variables)]
/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-data-csv_bin --bin rust-in-action-data-csv-main```
///
/// ```cargo doc  --package rust-in-action-data-csv_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-data-csv_bin ```
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
/// `nothing`
///
/// ## Example
///  `TODO`
///
///
fn main() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
      if i == 0 || record.trim().len() == 0 {
        continue;
      }

      let fields: Vec<_> = record
        .split(',')
        .map(|field| field.trim())
        .collect();
      if cfg!(debug_assertions) {
        eprintln!("debug: {:?} -> {:?}",
               record, fields);
      }

      let name = fields[0];

      if let Ok(length) = fields[1].parse::<f32>() {
          println!("{}, {}cm", name, length);
      }
    }
  }