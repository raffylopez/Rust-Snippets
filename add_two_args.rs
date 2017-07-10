// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
use std::env;

type SString = &'static str;

fn main() {
    let args: Vec<String> =  env::args().collect();

    let s1: &String = &args[1].;
    let ss1: Result<i32,_>= s1.parse();
    let s2: &String = &args[2];
    let ss2: Result<i32,_>= s2.parse();

    let num1 = match ss1 {
        Ok(n) => n,
        Err(_) => 0
    };

    let num2 = match ss2 {
        Ok(n) => n,
        Err(_) => 0
    };

    println!("The sum is {}", num1 + num2);
}
