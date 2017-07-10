// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

trait Foobar<T: Cool + Bar> { fn foobar()->T; }
trait Cool { fn cool(); }
trait Bar { fn bar(); }

struct Pokemon(i32);
impl Cool for Pokemon { fn cool() {println!("cool!");} }
impl Bar for Pokemon { fn bar() {println!("bar!");} }

struct JustCool(i32);
impl Cool for JustCool { fn cool() {println!("just cool!");} }

// -- yup, Pokemon implements both Cool and Bar
struct Robot(f32);
impl Foobar<Pokemon> for Robot {
    fn foobar()->Pokemon {
        Pokemon(111)
    } 
}

// -- nope, JustCool implements only Cool
// struct Robo(i32);
// impl Foobar<JustCool> for Robot {
//     fn foobar()->Pokemon {
//         Pokemon(111)
//     } 
// }

fn main() {

}
