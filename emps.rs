// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

#[derive(Debug)]
struct Department {
    name: &'static str
}

#[derive(Debug)]
struct Employee<'a> {
    name: &'static str,
    dept: &'a Department
}

fn main() {
    let s = Department {name: "Sales"};
    let e = Employee { name: "John", dept:&s};
    let m = Employee { name: "Mary", dept:&s};

    println!("s: {:?}", s);
    println!("e: {:?}", e);
    println!("m: {:?}", m);

    println!("e.dept: {}", e.dept.name);
    println!("m.dept: {}", m.dept.name);


}
