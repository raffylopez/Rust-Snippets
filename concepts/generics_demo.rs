// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;
use std::thread;
// use std::ops::*;

struct Pair<T> {
    one: T,
    two: T
}

fn gen_fun<T>(t:T)->T {
    t
}

fn gen_fun2<T>(pair: Pair<T>)->Pair<T> {
    pair
}

#[derive(Debug)]
enum Organism<T> {
    Human(T),
    Animal(T),
    Plant(T),
    Alien
}

fn takes_only_positive(num: i32)->Result<i32, &'static str> {
    if num < 0 {
        Err("nope.")
    } else {
        Ok(num * 2)
    }
}

trait Fooable{
    fn do_something(&self);
}

impl Fooable for String {
    fn do_something(&self) {
        println!("self: {:?}", self);
    }
}
impl Fooable for &'static str {
    fn do_something(&self) {
        println!("self: {:?}", self);
    }
}
impl Fooable for i32 {
    fn do_something(&self) {
        println!("self: {:?}", self);
    }
}

fn take_fooable<X: Fooable>(x: X) {
    x.do_something();
}

fn main() {
    let pair_of_nums = Pair {one: 1, two: 2};

    let t1 = thread::spawn(|| {println!("foobar!");});
    t1.join();

    let o: Organism<_> = Organism::Human("Joe");
    println!("o: {:?}", o);
    let res1 = takes_only_positive(-1);
    let res2 = takes_only_positive(123);

    println!("res1: {:?}", res1);
    println!("res2: {:?}", res2);


    let res= match res2 {
        Ok(n) => n,
        Err(s) => -1
    };

    let m = res;
    println!("res: {}", res);

    take_fooable("foo");
    take_fooable(123);

    // let x: Fooable = "foo";
    let n:Composed<Pokemon>= Composed(Pokemon(123));
}

struct Composed<T> (T);
struct Pokemon(i32);
impl Fooable for Pokemon {
    fn do_something(&self) {
    }
}
