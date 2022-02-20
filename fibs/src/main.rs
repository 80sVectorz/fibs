use std::env;
use num::bigint::BigUint;
use std::mem::replace;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut n_its: i32 = 10;
    
    if args.len() > 1 {
        nIts = args[1].parse().unwrap();
    }

    let mut a: BigUint = BigUint::from(0u32);
    let mut b: BigUint = BigUint::from(1u32);
    let start = Instant::now();
    for i in 1..n_its {
        let c = a + &b;
        a = replace(&mut b, c);
        println!("{}: {}",i, b);
    }
    let duration = start.elapsed();
    println!("Execution time: {}", duration);
    return;
}

