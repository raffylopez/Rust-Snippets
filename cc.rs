// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]

use Foo::{Bar,Baz};

// macro_rules! s (
//     (matcher) => (
//         
//     )
// )

type SString = &'static str;

enum Foo {
    Bar,
    Baz
}

enum Polygon<T> {
    Circle(T),
    Triangle(T),
    Square,
}

fn askOfYou(msg: String, )
fn main() {
    let mystring: SString = "foo";
    let x: Foo = Bar;
    let m: Polygon<SString> = Polygon::Circle("of Ossus");


}
