// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

trait DoSomething {
    fn do_something(&self) {
        println!("Doing something!");
    }
}

// -- even references are considered types, and
// -- can therefore implement traits as well!
impl <'a>DoSomething for &'a i32 {

}



fn main() {
    // let mut &rs:&mut i32 = &mut 123;
    // 


    let foo = &123;

    foo.do_something();
    let &bar = foo;
    let x:i32 = bar;
    println!("bar: {}", bar);
}
