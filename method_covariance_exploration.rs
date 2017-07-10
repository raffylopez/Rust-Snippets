// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

trait Fight {
    fn fight()->i32;
}

struct Pokemon(i32);
struct StreetFighter(i32);


struct Composite<T: Cool + Bar>(T);
trait Cool {}
trait Bar  {}

impl Fight for StreetFighter {
    fn fight()->i32 {
        321
    }
}

impl Fight for Pokemon {
    fn fight()->i32 {
        321
    }
}

fn main() {

}
