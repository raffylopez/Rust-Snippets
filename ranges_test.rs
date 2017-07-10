// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

fn main() {
    let mut myrange = 0..20;
    println!("myrange: {:?}", myrange);
    for i in myrange.clone() {
        println!("i: {}", i);
    }


    fn foo<S: std::ops::Deref>(d: S) {

    }

    let ls: Vec<_> = vec!["foo", "bar", "baz"];
    for item in &ls {
        println!("item: {}", item);
    }

    loop {
        let i = myrange.next();
        match i {
            Some(n)=> println!("n: {}", n),
            None=> break
        }
    }

    let rangea = 0..200;
    let res0: Vec<i32> =rangea.collect();
    for i in res0 {
        println!("i: {}", i);
    }

    // myrange.iter().map(|i| {println!("i: {}", i);});
}
