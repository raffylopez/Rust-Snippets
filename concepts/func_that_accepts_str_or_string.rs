// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

fn receive_str<S>(s: S) where S: Into<String> {
    let s:String = s.into();
}

fn main() {
    
}
