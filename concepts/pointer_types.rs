// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

#[derive(Debug)]
struct Pokemon(i32);

#[derive(Debug)]
struct StreetFighter<'a> {
    name: &'a str,
    hp: i32
}

fn main() {
    let m = 456;
    let n = 123;
    let mut p = Pokemon(444);
    { // -- read-only borrow (immutable borrow)
        let refp = &p;  
    }

    { // -- rw borrow (mutable borrow)
        let mrefp = &mut p;  
    }

    { // -- box (heap-allocated, one-owner)
        let b = Box::new(Pokemon(555));
    }

    { // -- rc : shareable box (heap-allocated, multiple owners)
        let rc = std::rc::Rc::new(p);
    }

    { // -- arc : thread-shareable / atomic reference-counted
        let ps = Pokemon(101);
        let ac = std::sync::Arc::new(ps);
    }

    { // -- raw ptr: unsafe, read-only
        let mut ps = StreetFighter{name: "ken", hp: 75};
        let uh_oh: *const StreetFighter = &ps;
        println!("uh_oh: {:?}", uh_oh);
        unsafe {
            println!("*uh_oh: {:?}", *uh_oh);
        }
    }

    { // -- raw mutable pointer
        let mut ps = StreetFighter{name: "ken", hp: 75};
        let uh_oh: *mut StreetFighter = &mut ps;
        println!("uh_oh: {:?}", uh_oh);
        unsafe {
            (*uh_oh).hp = 2000;

        }
    }

    let mut pika = Pokemon(111);
    pika.0 = 123;

    let mut pika_borrow = &mut pika;
    pika_borrow.0 = 456;
}
