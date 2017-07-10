// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

fn main() {
    let mut r: Vec<_> = vec!["atlanta", "california"];
    r[0] = "new york";
    println!("r: {:?}", r);
}
