// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]

type SString = &'static str;

fn get_input(s: SString)->i32 {
    println!("{}", s);
    let mut buf: String = String::new();
    let _iresult2 = std::io::stdin().read_line(&mut buf);
    let result: Result<i32, _> = buf.trim().parse();
    let ret:i32 = match result {
        Ok(v)=>v,
        Err(_)=>panic!("uh oh.")
    };
    ret
}

fn main() {
    fn inner_function() { }
    let result = get_input("Input a num: ");
    println!("Twice of that num is {}", result *2);
}
