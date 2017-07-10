// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]

use std::f32::consts;

#[derive(Debug)]
struct Employee(i32);

impl Drop for Employee {
    fn drop(&mut self) {
        println!("Dropping {}...", self.0 );
    }
}


static GAME_NAME: &'static str = "Plants vs. Aliens";
static mut COUNT: i32 = 123;
type SString = &'static str;



//  ,-----[ just a series of experiments ]
//  |
//  | indeed!
//  |
//  `-------

fn main() {
    println!("{}", GAME_NAME);
    println!("{}", std::f64::consts::PI);
    let foo: i32 = 12345_i32;
    println!("Binary foo: {:b}", foo);
    let foo_in_hex = format!("{:x}", foo);
    println!("Hexy foo: {}", foo_in_hex);
    println!("Pi : {:.*}", 2, std::f64::consts::PI);
    println!("Pi : {:+.3}", std::f64::consts::PI);

    let energy = 1.50_f64;
    let employee = Employee(123);
    let empty:() = ();

    unsafe {
        COUNT += 1;
    }
    println!("before shadow");
    let employee: Employee = Employee(456);
    println!("after shadow");  // -- shadowing does not free shadowed obj

    drop(employee);

    // -- shadowing 
    let energy = 500_i32;

    let e = Employee(111);
    // -- returning from a block
    let result = {
        let e_new_owner = e;
        println!("howyadoin'");
        e_new_owner
    };
    // -- shadowing can help simulate dynamic typing;
    // -- values don't have to have same type as shadowed
    let e = "foobar";

    println!("Result: {:?}", result);
    println!("end of program");

    let lithium = 4 as i32;
    let mut a = 5;
    let mut b = 6;
    let n = 7;
    let a = b = n;
    println!("{:?}", a);

    let a = 123;
    let b = &a;
    let c = &a;
    println!("{:p} and {:p}",b, c );
    println!("{}",b );

}


