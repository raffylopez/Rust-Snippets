// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]

use std::io;

type SString = &'static str;

fn main() {
    println!("Name? ");
    let mut mystr: String = String::new();
    let input = std::io::stdin().read_line(&mut mystr);
    println!("Greetings, {}!", mystr.trim() );

}
