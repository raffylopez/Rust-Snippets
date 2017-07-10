// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

// impl std::ops::Deref for Fn()->i32 {
//     fn deref(self)->Box<Fn()->i32> {
//         Box::new(self);
//     }
// }

fn curry(bf: Box<Fn()->i32>)->Box<Fn(i32)->i32> {
    let newbox = move |x: i32| {bf()};
    Box::new(newbox)
}

fn exp_compose(bf: Box<Fn(i32)->i32>)->Box<Fn(i32)->i32> {
    let newbox = move |x: i32| {bf(x) * bf(x)};
    Box::new(newbox)
}

fn main() {
    let curried = curry(Box::new(||{ 123})); // -- partial func!
    let res0 = curried(20);
    println!("res0: {}", res0);
    let twice = |i: i32| { 2 * i };
    let twice_and_exp = exp_compose(Box::new(twice));
    println!("twice_and_exp(5): {}", twice_and_exp(5));
}
