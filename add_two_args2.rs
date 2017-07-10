// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
use std::env;

type SString = &'static str;

fn main() {
    let args: Vec<String> =  env::args().collect();

    let addends: Vec<i32> = args.iter().map(|e: &String| { 
        let t: Result<i32,_> = e.parse();
        let nx = match t {
            Ok(n) => n,
            Err(_) => 0
        };
        nx
    }).collect();
    let sum: i32 = addends.iter().sum();

    println!("The sum is {}", sum);
}
