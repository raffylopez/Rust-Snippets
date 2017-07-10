// no-shebang
#![allow(unused_imports, dead_code, unused_variables, unused_must_use, unused_mut)]

type SString = &'static str;

#[derive(Debug)]
struct Pokemon {name: &'static str}


fn main() {
    let mut x = 5;
    let y = &mut x;
    *y = 123;

    let mut p: &mut Pokemon = &mut Pokemon {name:"Pikachu"};
    *p = Pokemon {name:"Foo"};

    // -- destructure example
    let pp = Pokemon { name: "Blastoise"};

    let Pokemon { name: x} = pp;
    println!("{}", x);

    let px = Pokemon {name: "Rabbite"};
    let ppx = px;
}
