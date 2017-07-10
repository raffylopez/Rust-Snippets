// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

fn cube(n: i32)-> i32 {
    n*n*n
}

fn main() {
    let range = 1..6;
    let nums = range.map(|n| cube(n));
    let nums:Vec<i32> = nums.collect();
    let sum = nums.iter().fold(1,|memo,a| memo * a);
    println!("sum: {}", sum);
}
