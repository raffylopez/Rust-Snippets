// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

#[derive(Debug)]
struct Country(String);

struct Foo<T>(T);

impl Into<String> for Country {
    fn into(self)->String {
        self.0
    }
}

impl Country {
    fn new<S> (n: S )-> Country where S: Into<String> {
        Country(n.into())
    }
    fn new_orig(name: &str)-> Country {
        Country(name.to_string())
    }
}

impl Into<Country> for String {
    fn into(self)->Country {
        Country(self)
    }
}

fn main() {
    let m = Country::new("Macau");
    let m = Country::new ("Macau".to_string());

    let converted:String = m.into();
    println!("converted: {}", converted);

    let s: String = "Bulgaria".to_string();
    let cc: Country = s.into();
    println!("cc: {:?}", cc);



}
