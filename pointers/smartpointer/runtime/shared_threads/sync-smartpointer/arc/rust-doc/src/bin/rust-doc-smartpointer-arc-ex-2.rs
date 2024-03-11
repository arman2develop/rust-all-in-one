#![allow(dead_code, unused_variables)]
use std::sync::{Arc, Weak};

/// rust-doc-smartpointer-arc-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-smartpointer-arc_bin --bin  rust-doc-smartpointer-arc-ex-2```
///
/// ```cargo doc  --package rust-doc-smartpointer-arc_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-smartpointer-arc_bin```
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
/// `TODO`


fn main(){



struct Gadget {
    me: Weak<Gadget>,
}

impl Gadget {
    /// Construct a reference counted Gadget.
    fn new() -> Arc<Self> {
        // `me` is a `Weak<Gadget>` pointing at the new allocation of the
        // `Arc` we're constructing.
        Arc::new_cyclic(|me| {
            // Create the actual struct here.
            Gadget { me: me.clone() }
        })
    }

    /// Return a reference counted pointer to Self.
    fn me(&self) -> Arc<Self> {
        self.me.upgrade().unwrap()
    }
}

}
