// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]

use std::env;
type SString = &'static str;

fn main() {
    let the_args:Vec<String> = env::args().collect();
    for the_arg in the_args.iter() {
        println!("{}", the_arg);
    }
}
