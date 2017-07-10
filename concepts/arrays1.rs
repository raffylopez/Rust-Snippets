// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

#[derive(Debug,Copy,Clone)]
struct Employee { name: &'static str }

#[derive(Debug,Copy,Clone)]
struct Credentials {credentials: &'static str }

struct Member {
    credentials: Credentials
}

impl Employee {
    fn new(name: &'static str)->Employee {
        Employee {name: name}
    }
}

fn main() {
    let fruits = ["apple", "grapes", "banana"];
    for i in fruits.iter() {
        println!("i: {}", i);
    }

    let nums: Vec<i32> = vec![23,14,21];
    for n in nums.iter() {
        println!("n: {}", n);
    }

    let luffy = Employee::new("luffy");
    let roronoa = Employee::new("roronoa");

    let mut employees: [Employee; 2] = [luffy, roronoa];
    let employees_refs: [&Employee; 2] = [&luffy, &roronoa];


    // -- as long as variable is declared mut, the array's elements can be changed; but
    // -- the array is no longer growable
    employees[0] = Employee::new("Sanji");

    println!("employees: {:?}", employees);

}

