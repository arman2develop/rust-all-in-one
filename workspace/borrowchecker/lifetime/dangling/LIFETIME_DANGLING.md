[[CONVERT]]
[[ria-for]]
[[rd-str]]

---


> The main aim of lifetimes is to prevent dangling references.
> which has an outer scope and an inner scope.


```rust
fn main() {
//    let reference_to_nothing = dangle();
let reference_to_nothing = no_dangle();
}
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
//primitive types need to &'a or &'static 
  fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
```
> no_dangle: This works without any problems. Ownership is moved out, and nothing is deallocated.
