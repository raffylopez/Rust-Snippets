// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

#[derive(Debug)]
struct Pokemon<'a> (&'a mut String);

impl <'a>Drop for Pokemon<'a> {
    fn drop(&mut self){
        println!("Freeing Pokemon {} into the wild!", self.0)
    }
}

fn main() {
    // -- when a struct is mutable, it means that you normally modify it in place, or
    // -- pass it to a function that modifies it directly (think StringBuffer);
    // -- in other words, its internals are free to be modified

    let mut pstring: &mut String = &mut "Pikachu".to_string();
    let pikachu = Pokemon(&mut pstring);  // -- pikachu is owner, pokemon struct is immutable

    // -- 
    let mut bstring: &mut String = &mut "Blastoise".to_string();
    let mut blastoise = Pokemon(&mut bstring);  // -- blastoise is owner, 
    {
        let blastoise_borrower = &blastoise;    // -- read-only borrower
    }

    {
        let blastoise_modder = &mut blastoise;  // -- blastoise-modder
        blastoise_modder.0.push_str("foo");
    }

    println!("blastoise: {:?}", blastoise);

}
