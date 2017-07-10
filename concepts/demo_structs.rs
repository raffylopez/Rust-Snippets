// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]

type SString = &'static str;


fn main() {
    // tuple destructurization
    let mt = ("foo", 123);
    let (x,y) = mt;
    println!("{}", x);
    println!("{}", y);

    // in-method tuple-struct destructurization
    struct Point(i32, i32);
    impl std::fmt::Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter)->std::fmt::Result {
            write!(f, "{} & {}", self.0, self.1)
        }
    }

    let np = Point(1,23);
    let Point(x,y) = np;
    println!("{}", np);
}
