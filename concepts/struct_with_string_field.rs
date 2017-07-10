// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

#[derive(Debug)]
struct Pokemon<'a> {
    name: &'a str,
}

fn main() {
    let p = Pokemon { name: "Pikachu "};
    let s: &str = "foo";

    println!("p: {:?}", p);
}

