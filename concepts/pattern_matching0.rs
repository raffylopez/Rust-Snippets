// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

// fn return_closure()-> (Fn(i32)->i32) {
//     panic!("foo");
// }

trait Atlas: Sized {
}

fn rc (a: i32)-> Box<Fn(i32)->i32> {
    Box::new(move |i32| {a})
}

trait ISortable {
    fn foo() {
        println!("cool!");
    }
}

fn process<T>(bx: Box<T>) where T: ISortable {
    {}
}

struct Employee(i32);

fn main() {
    let num = 223;
    let result = match num {
        0=> 123,
        200=>144,
        5|2=>124,
        100...110 => 100,
        _=>0
    };
    // let x:() = |i32| {println!()};
    // 
    let res0 = rc(111);
    let res1 = res0(123);
    println!("res1: {}", res1);


}
