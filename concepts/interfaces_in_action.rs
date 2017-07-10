// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

trait Fragmentable<T> {
    fn fragments(&self)->Vec<T>; 
}

impl Fragmentable<char> for &'static str {
    fn fragments(&self)->Vec<char> {
        self.chars().collect()
    }
}

#[derive(Debug,Copy,Clone)]
struct Contact(&'static str);

#[derive(Debug)]
struct PhoneBook {
    contacts: Vec<Contact>
}

impl <'a>Fragmentable<Contact> for PhoneBook {
    fn fragments(&self)->Vec<Contact> {
        self.contacts.clone()
    }
}



type SString = &'static str;

fn main() {
    println!("starting...");
    let v = "foo".fragments();
    println!("v: {:?}", v);
    let r=[1,23];
    println!("done.");
    // let x: Vec<String> = vec!["foo"];
    let pb = PhoneBook { contacts: vec![Contact("Tracy Jones"), Contact("Clannad")] };
    let cs = pb.fragments();
    println!("cs: {:?}", cs);


}
