#[allow(unused_imports)]
use std::thread;
#[allow(unused_imports)]
use std::fmt::{Display,Formatter};

#[allow(dead_code)]
#[derive(Debug)]
struct Ability {
    name: String,
    desc: String
}
// impl Display for Ability {
//     fn fmt(&self, f: &mut Formatter)-> fmt::Result {
//         write!(f, "foo")
//     }
// }
//
// impl Display for Vec<Ability> {
//     fn fmt(&self, f: &mut Formatter)-> fmt::Result {
//         write!(f, "foo")
//     }
// }
//
#[allow(dead_code)]
#[derive(Debug)]
struct Pokemon {
    abilities: Vec<Ability>,
    name: String,
    hp: i32
}

#[allow(dead_code)]
impl Pokemon {
    fn new<'a>(name: &'static str, hp: i32, abilities: Vec<Ability>) -> Pokemon {
        Pokemon {name: name.to_string(), hp: hp, abilities: abilities }
    }
}

#[allow(unused_variables)]
#[allow(unused_must_use)]

fn main() {
    let abilities: Vec<_> = vec![
        Ability {name:"Thunderbolt".to_string(),desc:"df".to_string()},
        Ability {name: "Taze".to_string(), desc:"foo".to_string()}
    ];

    let p  = Pokemon::new("Pikachu", 250, abilities);
    let collected: Vec<_> = p.abilities.into_iter().map(|elem|{println!("{:?}", elem)}).collect();
    println!("Done!");



}
