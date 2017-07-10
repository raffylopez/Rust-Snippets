// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

#[derive(Debug)]
struct Pokemon(i32);

fn moddit(f: &mut Pokemon) {
    f.0 = 1111;
}


fn main() {
    let mut x = 123;
    let y = &mut x;
    *y = 456;

    let mut p = Pokemon(444);
    moddit(&mut p);
    println!("p: {:?}", p);
    {
        let pp = &mut p;
        let qq = pp;
        qq.0 = 200;
    }

    println!("p: {:?}", p);

    // -- deref
    let mut carbon = 123;
    match carbon {
        mut r => println!("r: {:p}", &r)
    };

}
