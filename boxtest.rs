// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

struct Node {
    value: i32,
    next: Option<Box<Node>>
}
fn main() {
    let x = Box::new(123);
    let q = &x;
    let r = &*x;
    println!("r: {:p}", r);

    let a = q;

    let n = Box::new(42); 
    let mut m = n;
    *m = 67; // println!("{}", n); // error: use of moved value: `n` 
    println!("{}", m); // 67


    struct Pokemon(i32);
    fn get_pokemon(p: &Pokemon) {
    }

    let bp = Box::new(Pokemon(123));
    get_pokemon(&bp);

    let mut node = Box::new(Node{value: 200, next: None});
    let node2 = Box::new(Node{value: 199, next: Some(node)});
}
