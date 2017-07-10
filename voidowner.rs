// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

// -- Deref makes it possible for a Struct (non-ptr) to be dereferenced
impl std::ops::Deref for Pokemon {
    type Target = i32;

    fn deref(& self)->&Self::Target {
        &self.0
    }
}

#[derive(Debug)]
struct Pokemon(i32);
fn main() {
    let x = &Pokemon (123);
    let y = x;
    println!("y: {:?}", y);
    println!("x: {:?}", x);

    let x = &Pokemon (123);  // -- main owns the Pokemon
    // ~ let y = *x;              // -- y may not move ownership from main, because x is borrowing it! 
    // println!("y: {:?}", y);
    // println!("x: {:?}", x);

    let x = Pokemon (123);  // -- x owns the Pokemon
    let y = x;              // -- transfer of ownership ok!

    let x = Pokemon (123);  // -- main owns the Pokemon
    let y = &x;
    let z = **y;           // -- only made possible with Deref trait
    println!("z: {}", z);

    // -- this is cool! when a method is called on an object with deref,
    // -- it is automatically dereferenced to its value!
    println!("z + 200: {}", z + 200);
}
