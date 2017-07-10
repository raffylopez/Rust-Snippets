use std::thread;

#[allow(dead_code)]
#[derive(Debug)]
struct Thing {
    id: i32,
}

impl std::fmt::Display for Thing {
    fn fmt(&self, f: &mut std::fmt::Formatter)->std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[allow(unused_variables)]
#[allow(unused_must_use)]
fn main() {
    let myfn  = || {println!("foo!")};
    let myfn2  = |t: &mut Thing| {println!("foo!")};
    let handle = thread::spawn(myfn);
    let tt = Thing {id: 456};

    let handle2 = thread::spawn( move || {
        println!("cool!");
        let x = &tt;
        println!("{}", x.id);
    });

    // println!("{}", tt.id);
    let _result = handle.join().unwrap();
}
