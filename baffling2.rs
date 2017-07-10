// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

#[derive(Debug)]
struct Pokemon<T>(T);

impl <T>std::ops::Deref for Pokemon<T>{
    type Target = T;
    fn deref(&self)->&Self::Target { 
        &self.0
    }
}

impl <'a, T>std::ops::Deref for &'a Pokemon<T>{
    type Target = T;
    fn deref(&self)->&Self::Target { 
        &self.0
    }
}

fn main() {
    let x = &&&&20;
    println!("x: {}", x);
    let p = Pokemon(123);
    println!("p: {:?}", *p);
    let pp = &Pokemon(123);
    println!("pp: {:?}", &p);
}
