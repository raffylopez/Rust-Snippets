// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

fn main() {
    // -- ...looks like string comparisons are transparent

    let s1 = "Pikachu";
    let s2 = "Pikachu";
    let mut s3 = String::with_capacity(24);
    s3.push_str("Pikachu");

    if s1 == s2 {
        println!("s1 and s2 are equal");
    } else {
        println!("s1 and s2 are NOT equal");
    }

    if s1 == s3 {
        println!("s1 and s3 are equal");
    } else {
        println!("s1 and s3 are NOT equal");
    }

    if s1[..] == s3[..] {
        println!("s1 and s3 are equal");
    } else {
        println!("s1 and s3 are NOT equal");
    }

    if &s1[..] == &s3[..] {
        println!("s1 and s3 are equal");
    } else {
        println!("s1 and s3 are NOT equal");
    }
}
