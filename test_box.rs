// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

#[derive(Debug)]
struct Pokemon(&'static str);
impl Drop for Pokemon {
    fn drop(&mut self){
        println!("Freeing {} into the wild!", self.0)
    }
}

fn main() {
    let boxed = Box::new(123);
    let blastoise = Pokemon("Blastoise");
     
    let bp = Box::new(Pokemon("Pikachu"));
    println!("boxed: {:?}", boxed);
}
