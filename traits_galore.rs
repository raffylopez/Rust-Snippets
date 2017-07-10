// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

#[derive(Debug,Clone,Copy)]
struct Series(i32, i32);

impl std::ops::Add for Series {
    type Output = Series;
    fn add(self, with: Series)->Series {
        let n = self.0 + with.0;
        let d = self.1 + with.1;
        Series(n,d)
    }
}
struct Pokemon(i32);
fn foo<'a,T> (t: T, u: &'a Pokemon) {}

impl std::iter::Iterator for Series {
    type Item = i32;


    fn count(self)->usize {
        2
    }

    fn next(&mut self)->Option<Self::Item> {
        Some(122)
    }
}

fn main() {
    let f1 = Series(1,2);
    let f2 = Series(3,4);
    let f3 = f1 + f2;
    println!("f1: {:?}", f1);
    println!("f2: {:?}", f2);
    println!("f3: {:?}", f3);

    let mut f4 = Series(40,200);
    let fx = f4.next();
    println!("fx: {:?}", fx);
    println!("f1.next(): {:?}", f4.count());
}
