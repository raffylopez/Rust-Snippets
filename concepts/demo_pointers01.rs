// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]

type SString = &'static str;

#[derive(Debug)]
struct Pokemon{name: &'static str}

impl Drop for Pokemon {
    fn drop(&mut self) {
        println!("Freeing {} into the wild!", self.name );
    }
}
fn main() {
    let p = &mut Pokemon{ name: "Pikachu"};
    let pa = &p;
    let pb = &p;

    println!("pa and pb point to {0:p} and {1:p}", pa, pb );
    println!("pa and pb values are {:?} and {:?}", pa, pb );
    println!("pa and pb values are {:?} and {:?}", *pa, *pb );
    let mut blastoise = Pokemon {name: "Blastoise"};
    let m  = &mut blastoise;

    // repoint to diff obj
    let mut s = Pokemon {name: "Salamander"};
    drop(s);
    s = Pokemon {name: "Dragon"};

    let ss = &mut Pokemon {name: "Taury"};
}

