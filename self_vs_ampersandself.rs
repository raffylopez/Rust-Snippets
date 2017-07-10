// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

#[derive(Debug)]
struct Pokemon(i32);
impl Pokemon {
    fn borrow_fight(&self)-> i32 {
        println!("borrow_fight -> &self: {:?}", &self);
        123
    }

    fn owning_fight(self)-> i32 {
        println!("owning_fight -> self: {:?}", self);
        456
    }
}

fn main() {
    let p = Pokemon(123);
    let result =p.borrow_fight();
    let x = p;  //-- ok, self is just borrowed, so x can own


    let pa = Pokemon(456);
    let result =pa.owning_fight();  
    // let xa = pa; // -- nope, pa was swallowed by fn



}
