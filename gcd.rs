// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

fn gcd(a: f32, b: f32)-> f32 {
    if b == 0_f32 { 
        a 
    } else  {
        gcd(b, a % b)
    }
}

fn main() {
    println!("start");
    let pairs:Vec<(i32,i32)> = vec![(10,20), (30,50), (12, 36)];

    // -- we can use into_iter, since we want to move values for 
    // -- ownership
    // for i in pairs.into_iter() {
    //     // println!("i: {:?}", i);
    //     let (a,b): (i32,i32) = i;
    //     println!("a:{},  b: {}", a, b);
    // }

    // -- if we use iter, we get &(a,b), since iter only gives refs
    for i in pairs.iter() {
        let &(a,b): &(i32,i32) = i;
        println!("a={}, b={}, gcd={}", a, b, gcd(a as f32,b as f32));
    }

    for i in 1..10 {
        println!("i: {}", i);
    }

    // -- or we can use it with map
    pairs.into_iter().map(move|(e,f)|{e*f} );

    println!("ok");
}
