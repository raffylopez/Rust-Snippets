// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]

type SString = &'static str;

enum Gender<T> {
    Male(T),
    Female(T)
}
use Gender::{Male, Female};

fn main() {
    let jim = Male("Jim");
    let lisa = Female("Lisa");

    match jim {
        Male(a)=>println!("{} is male!", a),
        Female(a)=>println!("{} is female!", a)
    }

    let result = match lisa {
        Male(a)=>format!("{} is male!", a),
        Female(a)=>format!("{} is female!", a)
    };

    println!("{}", result);
}
