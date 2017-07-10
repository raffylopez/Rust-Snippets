// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

fn main() {
    let nums: Vec<_> = vec![5,23,4,10];
    for n in &nums {
        let y = *n + 1;
        println!("n: {}", n);
        println!("y: {}", y);
    }
}
