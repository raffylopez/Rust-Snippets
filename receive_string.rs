// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

fn twice_of_len(s: &String)-> usize {
    let size = s.len();
    size * 2
}

fn how_long(s: &str)->usize {
    s.len()
}

fn main() {
    let foo = twice_of_len(&"foobar".to_string());
    println!("foo: {}", foo);

    let res0 = how_long(&"foo"[..]);
    println!("res0: {}", res0);

}
