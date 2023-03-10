#![allow(dead_code, unused_variables)]

/*
Main

## Commands


```cargo run --package other-benches_lib --bin other-benches-ex-2```

```cargo doc  --package other-benches_lib --message-format short --no-deps --open --color always```

```cargo test --doc  --package other-benches_lib```

## What
`medium-dyn-static-no-inline`

## How
`TODO`

# Arguments

* `Arg1` - This is the [your type] to [your verb] the [your struct/func name]

# Return
`nothing`

## Example
 `TODO`
*/
/// //```rust,compile_fail,no_run,ignore
use std::env;
use std::time::SystemTime;

trait Backend{
    fn compute(&self,number: i64) -> i64;
}

struct PositiveBackend;
struct NegativeBackend;

impl Backend for PositiveBackend{
    #[inline(never)]
    fn compute(&self,number: i64) -> i64{
        number+1
    }
}

impl Backend for NegativeBackend{
    #[inline(never)]
    fn compute(&self,number: i64) -> i64{
        number-1
    }
}

fn main() {

    // decide which backend to use based on a user-set program argument
    let backend = match env::args().skip(1).next() {
        Some(x) => Box::new(PositiveBackend ) as Box<dyn Backend>,
        _ => Box::new(NegativeBackend) as Box<dyn Backend>
    };

    let mut res= 0 as i64;

    let start_time = SystemTime::now();

    let total = 20_000_000 as i64;

    for i in 0 .. total {
        res += backend.compute(i) + res;
    }

    println!("Result: {}",res);
    println!("Elapsed_ms: {}", start_time.elapsed().unwrap().as_millis());

}
