// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

trait IntoString {
    type InjectMe;
    fn into_string(&self); 
}

struct Pokemon (&'static str);
struct Car (&'static str);

impl IntoString for Pokemon {
    type InjectMe = &'static str;
    fn into_string(&self) {
        println!("pika!");
    } 
}

impl IntoString for Car {
    type InjectMe = &'static str;
    fn into_string(&self) {
        let x: Self::InjectMe = "foo";
        println!("car!");
    } 
}

// -- Foobar
trait Foobar {
    fn foo(&self) {
        println!("ok");
    }
}

// -- Something, surprising: if str implements Foobar, then type InjectMe = Foobar
// -- may also happen

impl Foobar for &'static str { } // attach; simulates subclassing, str is-a Foobar

fn receive_intostr<I>(arg: &I) where I: IntoString, I::InjectMe: Foobar {
    arg.into_string();
}

fn main() {
    let p = Pokemon ("Pikachu");
    let c = Car ("ford");
     
    receive_intostr(&p);
    receive_intostr(&c);

    // "cool".foo();
}
