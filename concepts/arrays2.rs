// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

fn main() {
    // +--------+
    // | Basics |
    // +--------+
    // -- Arrays can be declared and initialized this way
    let mut buffer = [0; 20];
    let mut buffer: [i32; 20] = [-1; 20];
    buffer[0] = 123;  // -- you'll get an error if [0;20] weren't assigned to buffer
    println!("buffer[0]: {}", buffer[0]);
    println!("buffer[1]: {}", buffer[1]);  

    // +-------------------+
    // | Pointer to Arrays |
    // +-------------------+
    let pk = ["Pikachu", "Blastoise", "Salamander"];
    let ppk = &pk;

    // -- pointers to arrays are automatically dereferenced, no need to *ppk
    println!("ppk[0]: {}", ppk[0]);
    println!("(*ppk)[0]: {}", (*ppk)[0]);

    // +----------------------+
    // | Mutability of Arrays |
    // +----------------------+
    // -- as long as variable is declared mut, the array's elements can be changed; but
    // -- the array is no longer growable
    let mut sports = ["Basketball", "Baseball"];
    sports[0] = "Polo";

    println!("sports: {:?}", sports);
    // +---------+
    // | Looping |
    // +---------+
    // -- iter() method
    for i in buffer.iter() {
        print!("{} ", i);
    }
    println!("");

    // -- classic, but slow
    for i in 0..buffer.len()-1 {
        print!("{}={}", i, buffer[i]);
    }

    // -- alternative, for can take a slice 
    for i in &buffer {
        print!("{} ", i);
    }
    println!("");

    // -- alternative, for can take a slice (can be &buffer[..])
    for i in &buffer[..] {
        print!("{} ", i);
    }
    println!("");
}
