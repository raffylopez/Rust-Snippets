// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

struct Pokemon(i32);

fn receiveref(i: &i32)->i32 {
    let x:i32 = *i;
    *i + 1
}

fn main() { 
    let x = 123;
    let y = 456;

    let res0 = receiveref(&x);
    println!("res0: {}", res0);
}
