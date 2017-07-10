// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

// -- prob is that passed fn cannot be placed into a boxed closure 
// -- that's returned. ff is, anyhow, a ref
fn receive_fn<'a,'b>(ff: &'b(Fn()->i32))->Box<(Fn()->i32)>{
    let xy = (*ff)();
    println!("xy: {}", xy);
    let clozure =  move || { (*ff)() * 2 };  // -- deref
    Box::new(clozure)
    // panic!("foo");
}

fn main() {
    let ff = || { 123 };
    receive_fn(&ff);
}
