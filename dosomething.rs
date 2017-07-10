// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

fn main() {
    let result = get_input("Number: ".to_string());
    println!("You provided: {}", result );
    let tp: Result<i32, _> = result.parse();
    let p:i32 = match tp {
        Ok(n) => n,
        Err(_) => 0
    };
    println!("Twice that number is {}", p * 2);
}

fn get_input_mut(mut buf: &mut String) {
    println!("Input: ");
    std::io::stdin().read_line(&mut buf);
    buf.trim();
}

fn get_input(msg: String)->String {
    println!("{}", msg);
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf);
    buf.trim().to_string()
}
