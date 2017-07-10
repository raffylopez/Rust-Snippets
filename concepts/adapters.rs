// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

fn main() {
    let ls: Vec<&str> = vec!["apple", "grapes", "apple", "banana"];
    // -- iter yields refs to each item, so we deref
    let rs = ls.iter().find(|x| { *x == &"apple" });
    println!("rs: {:?}", rs);



    let ls: Vec<i32> = vec![1,2,3,4];
    ls.iter().find(|n| { *n > &2 });  

    let rs:Vec<i32> = (1..20).collect();

    let rs0 = rs.iter().find(|n| *n > &10);

    let result = match rs0 {
        Some(&n)=>n,
        None=>0
    };

    println!("rs0: {:?}", rs0);


}
