use std::env;
use num::bigint::BigUint;
use std::mem::replace;
use std::time::Instant;

fn len_check(la: usize,lb: usize) -> bool{
  if la > lb{
    return true;
  } else{
    return false;
  }
}

fn help(){
    println!("
Fibs is a pure Rust CLI command for calculating fibonacci numbers
-----------------------------------------------------------------
fibs -h/--help | Prints this message.

Parameters/Flags:
-s or --start | The iteration for output.
-e or --end | The ending iteration for output.
-b or --benchmark | Run without printing.
    ");

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut iteration_start: i64 = 0;
    let mut iteration_end: i64 = 10;
    let mut benchmark: bool = false;
    if args.len() > 1{
        for i in 1..args.len(){
            if args[i].to_lowercase() == "-h" || args[i].to_lowercase() == "--help"{
                help();
                return;
            }
            if args[i].to_lowercase() == "-b" || args[i].to_lowercase() == "--benchmark"{
                benchmark = true;
            }
            if args[i].to_lowercase() == "-s" || args[i].to_lowercase() == "--start" {
                if len_check(args.len(),i+1){
                    if args[i+1].parse::<i64>().is_ok() {
                        iteration_start = args[i+1].parse::<i64>().unwrap();
                    } else {
                        println!("{} is not a number",args[i+1]);
                        return;
                    }
                } else {
                    println!("Please give number after the -s or --start flag");
                    return;
                }
            } else if args[i].to_lowercase() == "-e" || args[i].to_lowercase() == "--end" {
                if len_check(args.len(),i+1){
                    if args[i+1].parse::<i64>().is_ok() {
                        iteration_end = args[i+1].parse::<i64>().unwrap();
                    } else {
                        println!("{} is not a number",args[i+1]);
                        return;
                    }
                } else {
                    println!("Please give number after the -e or --end flag");
                    return;
                }
            }
        }
    }else {
        help();
        return;
    }
    if iteration_start > iteration_end {
        println!("Please give a valid range");
        return;
    }
    let mut a: BigUint = BigUint::from(0u32);
    let mut b: BigUint = BigUint::from(1u32);
    let start = Instant::now();
    for i in 0..iteration_end {
        let c = a + &b;
        a = replace(&mut b, c);
        if !benchmark && i >= iteration_start {
            println!("{}: {}",i, b);
        }
    }
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
    }
