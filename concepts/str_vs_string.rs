// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

// +-----------------------------+
// | String vs. str: What to Use |
// +-----------------------------+

// -- NOTES:
// -- > A String is a vec[u8]
// -- > When compiler sees &String (or other types, with &_) the value is coerced into &str
// -- > Other types, include String, can coerce into &str
// -- > "foobar" actually gives &str
// -- > To deref, use &

fn recv_string(s: String) { // -- no need to do it this way; owning xfer
}


fn recv_string_as_ref(s: &String) { // -- no need to do it this way

}

fn recv_str(s: &str) {
    println!("s: {}", s);
}

type SString = &'static str;

fn main() {
    let mut s = String::with_capacity(24);
    s.push_str("pikachu!");
    recv_str(&s);

    let s: &str = &s;  // -- deref (String to &str) happens here, when using &

    let b: String = String::new();
    let c: &String = &String::new();
    recv_str(&b);


}
