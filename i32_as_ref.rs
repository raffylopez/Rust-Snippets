// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

fn add(num1: &i32, num2: &i32)->i32 {
    num1 + num2
}

fn main() {
    let result = add(&1,&23); // -- caveat: messy syntax
    println!("result: {}", result);
}
