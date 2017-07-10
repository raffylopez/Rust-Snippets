// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]

type SString = &'static str;

fn main() {
    let mut one: String = String::new();
    let mut two: String = String::new();

    println!("First: ");
    let _iresult1 = std::io::stdin().read_line(&mut one).ok().expect("foo");
    println!("Second: ");
    let _iresult2 = std::io::stdin().read_line(&mut two);

    let a: Result<i32, _> = one.trim().parse();
    let b: Result<i32, _> = two.trim().parse();

    let result1 = match a {
        Ok(v) => v,
        Err(_) => 0
    };

    let result2 = match b {
        Ok(v) => v,
        Err(_) => 0
    };

    println!("Sum is: {}", result1 + result2 );


}
