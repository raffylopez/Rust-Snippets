// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

struct MyBox<T> {
    content: T
}

struct Chocolate (i32);
struct Toy (i32);

trait Flyer {
    fn fly() {
        println!("flying!");
    }
}

trait Fighter {
    fn fight() {
        println!("fighting!");
    }
}

fn take_box(bx: Box<Fighter + Send> ) {
}

fn main() {

}
