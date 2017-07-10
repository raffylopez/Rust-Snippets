// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;



fn main() {
    let pi32 = &25;
    println!("*pi32: {}", *pi32);
    println!("&pi32: {}", &pi32);
    println!("pi32: {}", pi32);
    println!("&*pi32: {}", &*pi32);
    let x: () = &*pi32;
}
