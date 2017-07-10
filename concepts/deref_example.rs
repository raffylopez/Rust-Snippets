// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]


type SString = &'static str;

#[derive(Debug)]
struct Pokemon(&'static str);

impl std::ops::Deref for Pokemon {
    type Target = str;

    fn deref(&self)->&Self::Target {
        "foobar"
    }
}

fn main() {
    let pikachu = Pokemon("pikachu");
    let dpikachu = &*pikachu;
    let dpikachu = &pikachu;
    println!("dpikachu: {:?}", &dpikachu);
}
