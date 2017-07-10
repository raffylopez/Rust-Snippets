// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]


type SString = &'static str;

#[derive(Debug)]
struct Pokemon<T>(T);

impl <T>std::ops::Deref<Target=T> for Pokemon<T> {
    type Target = T;

    fn deref(&self)->&Self::Target {
        &self.0
    }
}

fn main() {
    let pikachu = Pokemon("pikachu");
    // let dpikachu = &*pikachu;
    // let dpikachu = &*pikachu;
    println!("pikachu: {:?}", &pikachu);
}
