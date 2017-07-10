// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;


fn main() {
    // -- string slice
    let str3 = "foobar".to_string();
    let str33 = &str3[..];

    // -- static string slice
    let s = "one";
    let s2 = &s[..];

    println!("str33: {}", str33);

    let nums = vec![1,2,3,4];
    let view_into_nums = &nums[0..2];

    println!("Result: {:?}", view_into_nums);

    // -- no-overhead string comparison (use &target[..] or &target)
    {
        let s1 = "foobar";
        let mut s2 = String::with_capacity(24);
        s2.push_str("foobar");
        if s2 == s1 {
            println!("Uh-huh");
        } else {
            println!("Nope");
        }
        if &s2[..] == s1 {
            println!("Sure");
        } else {
            println!("Nope");
        }
    }

    let mut s1 = String::with_capacity(24);
    s1.push_str("cool");
    let mut s2 = String::with_capacity(24);
    s2.push_str("cool");
    println!("s1: {}, s2: {}", s1, s2);

    if s1 == s2 {
        println!("Yup with ==");
    } else {
        println!("Nope with ==");
    }
    if &s1 == &s2 {
        println!("Yup");
    }

    if &s1[..] == &s2[..] {
        println!("Yup");
    }

    // -- chars() method
    // 
    for i in "amazing".chars() {
        println!("i: {}", i);
    }

    // -- splitting
    let s: &'static str = "the quick brown fox";
    let pieces = s.split(" ");
    for p in pieces {
        println!("p: {}", p);
    }

    // -- replace 
    let mut replaceable = String::with_capacity(24);
    replaceable.push_str("amidst the sea of chaos");


    let replaced = replaceable.replace("sea", "ocean");
    println!("replaceable: {}", replaced);

    // -- fn
    let ss = "quick brown fox";
    let ssref: &'static str = &ss[0..5];

    // println!("sssref: {:?}",vr );
    let x = &replaceable.to_string();
    
}


fn receive_str(s: &'static str) {
}

fn count_spaces(s: &String) {
}

fn receive_i32_as_ref(num: &i32) {
}
