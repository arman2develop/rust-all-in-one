#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p educative-dangle_bin --bin educative-dangling-main```
///
/// ```cargo doc  --package educative-dangle_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package educative-dangle_bin```
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
/// `The longest string is long string is long`
/// `The longest string is long string is long`
///
/// ## Example
/// In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.
///
/// When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters.
///
/// If the reference returned does not refer to one of the parameters, it must refer to a value created within this function, which would be a dangling reference because the value will go out of scope at the end of the function.
///
/// ```rust,no_run
/// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
///    x
///}
///```
///
/// * `Important`
/// ```result.as_str()```
///
/// > no relation between params lifetime and return, so we won't need any lifetime generic.
///
/// ```rust,no_run,ignore
/// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
///     let result = String::from("really long string");
///     result
/// }
/// ```
///

/// ```rust,no_run,cmopile_fail
/// fn main() {
///     {
///         let a;
///
///         {
///             let b = 15;
///             a = &b; //Error Exposed
///         }
///
///         println!("a: {}", a);
///     }
/// }
/// ```
/// `output`
/// error[E0597]: `b` does not live long enough

fn main(){
    unimplemented!();
}
