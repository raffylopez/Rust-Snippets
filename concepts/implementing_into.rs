// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

struct Country {
    name: String,
    capital: String
}


impl Country {
    fn new(name: &str, capital: &str)->Country {
        Country {
            name: name.to_string(), 
            capital: capital.to_string()
        }
    }
}

impl Into<str> for Country {
    fn into(self)->T {
    }
}

fn main() {
    let mut p = Country::new("Philippines", "Manila");
    p.name = "Foobar".to_string();
}
