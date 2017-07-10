// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

#[derive(Debug)]
struct Dereferencing<T>(T);

use std::ops::Deref;


impl <T>Deref for Dereferencing<T> {
    type Target = T;
    fn deref(&self)->&T {
        &self.0
    }
}

struct Pokemon(i32);



trait Foo {
    fn foo(self);
}

// impl Foo for &Dereferenceing
//

fn main() {
    let drf = Dereferencing(123);
    let foo = &Dereferencing(456);
    println!("&drf: {:?}", *drf);
    println!("foo: {:?}", *foo);
}
