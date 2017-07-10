// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

// #[derive(Debug,Copy,Clone)]
#[derive(Debug)]
struct Pokemon {name: &'static str, level: i32}

fn main() {
    println!("Begin.");
    let p0 = Pokemon { name: "Pikachu", level: 25};
    let mut p = p0;

    {
        let mut p2 = &mut p;
        p2.name = "Psyduck!";
    }

    println!("{}", p.name);
    println!("Done.");

    let boxme = 123;
    let boxed = Box::new(boxme); // copied

    let test = true;

    if test {
        println!("yup");
    }
}

#[test]
fn test_pokemon() {
    let p = Pokemon {name: "Pikachu", level: 25};
    assert_eq!(p.name, "Pikachu");
}
