// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

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

fn main() {
    let s: &'static str = "foobar";
    let mut gs: String = String::new();
    gs.insert_str(0,"amazing");
    println!("gs: {}", gs);

    let mut hero = String::with_capacity(24);
    hero.push_str("Tarzan");

    let mut maiden = String::with_capacity(24);
    maiden.push_str("Jane");
    
    println!("Hero: {} and {}", hero, maiden);

    let mut buf = String::new();
    buf.push_str("foo");
    get_input_mut(&mut buf);
    println!("You typed: {}", buf);

    let result = get_input("Enter something: ".to_string());
    println!("{}", result);
}

