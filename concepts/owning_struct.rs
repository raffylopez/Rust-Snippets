// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

#[derive(Debug)]
struct Country {
    name: String, // --> (a)
    capital: String
}
// (a) --> implications: no & means that it is (1) owned by the struct; 
// (2) String obj (not name variable) is still mutable; (3)
// it's not a reference

fn main() {
    let mut c = Country { name: "Philippines".to_string(), capital: "Manila".to_string()};
    c.name = "Cool".to_string();
    println!("c: {:?}", c);
}
