type SString = &'static str;
#[allow(dead_code)]
type ThingWithLifetime<'a> = Thing<'a>;
#[allow(dead_code)]
type GenericInt = Genericity<i32>;

#[allow(dead_code)]
struct Genericity<T> {
    foo: T
}

#[allow(dead_code)]
struct Thing<'a> {
    id: &'a mut i32
}

#[allow(unused_variables)]
#[allow(unused_must_use)]
fn main() {
    let f = 1.23;
    let u = 1.23 as u32;
    let s: SString = "foo";
    let a: Thing;
}
