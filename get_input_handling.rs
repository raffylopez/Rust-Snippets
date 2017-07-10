// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

fn sqroot(num: f64)->Result<f64, String> {
    if num < 0.0 {
        return Err("Value can't be negative!".to_string());
    }

    Ok(num.sqrt())
}

fn find_it(search: String) -> Result<String, String> {
    if search == "cowsay" {
        return Ok("Found it".to_string());
    } 
    Err("nope".to_string())
}

fn main() {
    println!("Input: ");
    let mut ss = String::new();
    std::io::stdin().read_line(&mut ss);

    let foo: Result<i32, _> = ss.trim().parse();

    let r = match foo {
        Ok(v) => "cool",
        Err(_) => "not cool!"
    };
    
    println!("{:?}",r );

    let result = find_it("cowsay".to_string());
    println!("{:?}", result );

    let strontium = if true { 123 } else { 456 };
    println!("{}", strontium);

    let health = 0;

    let gamestate = if health <= 0 { "Game over!"} else {"" } ;
    let vresult = verbose(100);
    println!("{}", vresult);
}

fn verbose (x: i32) -> &'static str {
    if x < 10 {"less than 10 "} else { "10 or more"}
}
