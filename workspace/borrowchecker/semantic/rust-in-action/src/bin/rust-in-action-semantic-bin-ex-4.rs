#![allow(dead_code, unused_variables)]

/// rust-in-action-dangling-bin-ex-4
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-semantic_bin --bin  rust-in-action-semantic-bin-ex-4```
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
/// //```rust,no_run,compile_fail,ignore
//Avoiding ownership issues with references
//1sat
#[derive(Debug)]

struct CubeSat {
   id: u64,
   mailbox: Mailbox,
 }

 #[derive(Debug)]
 struct Mailbox {
   messages: Vec<Message>,
 }

 type Message = String;

 struct GroundStation;

 impl GroundStation {
     fn send(&self, to: &mut CubeSat, msg: Message) {
         to.mailbox.messages.push(msg);
     }
 }

 impl CubeSat {
     fn recv(&mut self) -> Option<Message> {
         self.mailbox.messages.pop()
     }
 }

 fn main() {
     let base = GroundStation {};
     let mut sat_a = CubeSat {
       id: 0,
       mailbox: Mailbox {
         messages: vec![],
       },
     };

     println!("t0: {:?}", sat_a);

     base.send(&mut sat_a,
               Message::from("hello there!"));

     println!("t1: {:?}", sat_a);

     let msg = sat_a.recv();
     println!("t2: {:?}", sat_a);

     println!("msg: {:?}", msg);
 }
