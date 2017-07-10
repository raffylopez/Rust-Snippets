// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

trait Fighter {
    fn action(&self);
}

struct Pokemon<'a>(&'a str);
struct StreetFighter<'a>(&'a str);

impl <'a>Fighter for Pokemon<'a> {
    fn action(&self) {
        println!("pika!");
    }
}

impl<'a>Fighter for StreetFighter<'a> {
    fn action(&self) {
        println!("hadouken!");
    }
}

fn fighto<S>(s: S) where S: Fighter  {
    s.action();
}

fn main() {
    fighto(StreetFighter("ken"));
    fighto(Pokemon("pikachu"));
    

}
