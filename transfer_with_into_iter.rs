// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;


#[derive(Debug,Clone)]
struct Power(i32);
#[derive(Debug,Clone)]
struct Pokemon(i32, Power);
fn main() {
    let ls: Vec<_> = vec![1,2,3,4,5];
    let xs = ls.clone();
    let res0 = ls.into_iter().find(|elem| elem % 2 == 0);
    println!("res0: {:?}", res0);
    println!("xs: {:?}", xs);
    let pa = Pokemon(123, Power(2000));
    let pb = pa.clone();  // -- deep copy
    println!("pb: {:?}", pb);

}
