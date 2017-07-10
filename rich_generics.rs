
// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

trait Fooable {
}

impl <'a>Fooable for &'a i32 { }

struct S1(i32);
struct S2(f32);

trait Draw {
    fn draw(&self);
}

impl Draw for S1 { fn draw(&self) { println!("***{}***", self.0); } }
impl Draw for S2 { fn draw(&self) { println!("***{}***", self.0); } }

fn draw_object<T: Draw> (t: &T) {
    t.draw();
}

fn rich_gen<T,U>(t: T, u: U) 
    where T: Fooable + std::ops::Deref {
    }

fn main() {
    rich_gen(&123, &"adsf");
    let s1 = S1(123);
    let s2 = S2(4456.0);
    draw_object(&s1);
    draw_object(&s2);
}
