#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p rust-doc-trait-inheritance_bin --bin rust-doc-trait-inheritance-main```
///
/// ```cargo doc  --package rust-doc-trait-inheritance_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-trait-inheritance_bin ```
///
/// ## What
/// `Supertrait-multi-trait`
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
///

trait Person {
    fn name(&self) -> String;
}

// Person is a supertrait of Student.
// Implementing Student requires you to also impl Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a subtrait of both Programmer
// and Student. Implementing CompSciStudent requires you to impl both supertraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

fn main() {

    unimplemented!();
}
