// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

use Outcome::{Success, Fail};


#[derive(Debug)]
enum Outcome<T> {
    Success(T),
    Fail
}

impl std::fmt::Display for Outcome<i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter)->std::fmt::Result {
        write!(f, "aloha")
    }
}

fn get_success<T>(item: T)->Outcome<T> {
    Success(item)
}


fn main() {
    let result = get_success(123);
    println!("result: {:?}", result);

    // deconstruct
    let derived = match result {
        Success(n) => n,
        Fail => 0
    };

    println!("derived: {}", derived);
}
