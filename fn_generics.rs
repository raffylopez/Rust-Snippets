// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

fn foobar<T>(p:T) {
    println!("foo");
}

fn curry2(ref num_gen:fn(i32)->i32)-> fn(i32)->i32 {
    *num_gen
}

// fn curry<'a> (f: &(Fn(i32)->i32))-> Box<(Fn(i32)->&(Fn()))> {
//     let res0 = &f(100);
//     println!("res0: {}", res0);
//     panic!("foo");
//     Box::new(|i:i32|{&f(i)})
// }

// -- a function that receives a method-local function into its
// -- enclosure
fn return_fn(foo:i32)-> Box<(Fn(i32)->i32)> {
    let x = 123;
    let y = foo;
    let ff = |x: i32| { 123 };
    Box::new( move |i:i32|{i * x * y * ff (1)})
}


// -- a function that takes a function
fn twice_and_then(i: i32, f: &(Fn(i32)->i32))-> i32 {
    f(i * 2)
}

fn receive_fn(bar: &Fn()->i32) {
    let v = bar();
}

// -- takes a function, returns a function
fn enclose<'a>(f: &'a(Fn(i32)->i32))->Box<(Fn(i32)->i32)> {
    let res0 = f(123);
    // -- won't be able to capture f, since f is a ref
    let fff = move |x: i32| { res0 };
    Box::new(fff)
}

// -- takes a non-capturing function, returns a function
fn receive_static_fn(f: fn(i32)->i32)->Box<(Fn(i32)->i32)> {
    let ff = move |x: i32| { f(x)};
    Box::new(ff)
}
// // -- takes a static function, returns a function
// fn receive_static_fn2(f: fn(i32)->i32)->(Fn(i32)->i32) {
//     let ff = |x: i32| { f(x)};
//     ff
// }

fn main() {
    let x = foobar::<f32>(2.0);
    // curry(&(|f| { 123}));
    // let myfn:fn(i32)->i32 = |i| {123};
    // let ident = curry2(&myfn);
    enclose(&|i: i32| {456});
    let res0 = twice_and_then(200, &|x: i32| {x+50});
    println!("res0: {}", res0);
}
