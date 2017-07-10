
// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]

static GREETING:&'static str="welcome!";
static NUM: i32 = 123;
use std::env;
type SString = &'static str;

fn main() {
    let osvars = env::vars();
    for (k,v) in osvars {
        println!("{} -> {}", k,v);
    }
}
