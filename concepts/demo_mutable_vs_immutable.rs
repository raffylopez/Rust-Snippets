// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

#[derive(Debug)]
struct Pokemon{name: &'static str}

impl Drop for Pokemon {
    fn drop(&mut self) {
        println!("Freeing {} into the wild!", self.name );
    }
}
type SString = &'static str;

fn main() {
    // -- (1) immutable variable, immutable object
    let p = Pokemon { name: "Pikachu" };

    // -- can't assign anything new to p
    // p = Pokemon { name: "Poring" }

    // -- (2) variable can't point to anything else, structure is actually borrowed here
    let m = &mut Pokemon { name: "Megalodo"};
    m.name = "Megalodragon";

    // -- (3) variable can point to another Pokemon, but Pokemon isn't modifiable;
    let mut k = Pokemon { name: "Tunafish" };

    // -- (4) variable can point to elsewhere, struct is borrowed
    let mut l = &mut Pokemon { name: "Luna" };
    drop(l);
    
    // println!("{:?}", l); // -- no longer available; "consumed" by method

    // -- (5) variable can point to elsewhere, immutable borrow (read-only struct)
    let mut l = &Pokemon { name: "Luna" };


}
