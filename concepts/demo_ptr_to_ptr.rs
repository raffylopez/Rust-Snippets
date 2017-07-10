// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

#[derive(Debug)]
struct Pokemon{name: &'static str}

impl Drop for Pokemon {
    fn drop(&mut self) {
        println!("Freeing {} into the wild!", self.name );
    }
}
type SString = &'static str;

fn foo(ref x: i32) {
}

fn main() {
    let pk = Pokemon {name: "Pikachu"};
    let ref ppk = &pk;
    let ref p_pk = &ppk;

    println!("?:[pk]   pk is bound to {:?}", pk );
    println!("[*pk]    pk cannot be dereferenced, it's not a ptr and is statically bound to Pokemon!");
    println!("[&pk]    'address of' pk {:p}", &pk );
    println!("");
    println!("[ppk]    ppk is a pointer to pk");
    println!("[ppk]    ppk, when println'd, has value of {:?}", ppk );
    println!("[ppk]    ppk points to addr {:p}, same as &pk", ppk );
    println!("[*ppk]   the dereference value of ppk is {:?}", *ppk );
    println!("[*ppk]   the dereference of ppk, *ppk, is not a pointer, so cannot be (:p)'d");
    println!("");
    println!("[&ppk]   ppk points to addr {:p}", &ppk );
    println!("[p_pk]   p_pk points to addr {:p}", p_pk );
    println!("[*p_pk]  p_pk has dereference of {:?}", *p_pk );
    println!("");

    let x = Pokemon{name:"Xenon"};
    // ~ let ref mut y = &x;
    // ~ let ref mut z = *y;
    // 
    let ref ppx = &&x;  // -- pointer to pointer
    println!("{:?}",**ppx); // -- deref twice, so we get static Pokemon back
}
