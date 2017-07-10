// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

struct Pokemon(i32);
struct StreetFighter(i32);

trait Fighter:Sized {}

impl Fighter for Pokemon{}
impl Fighter for StreetFighter{}

fn main() {
    let x = Pokemon(122);
    let y = StreetFighter(123);
    <T>{
    }
}
