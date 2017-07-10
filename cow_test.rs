// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

use std::borrow::Cow;

fn main() {
    let foo: Cow<str> = Cow::Borrowed("foo");
    let bar: Cow<str> = Cow::Owned("foo".to_string());
    println!("foo: {}", foo);
    println!("foo: {}", bar);


}
