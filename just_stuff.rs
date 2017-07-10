// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

struct Pair<T> { one: T, two: T }

impl <T>std::string::ToString for Pair<T> {
    fn to_string(&self)->String {
        "foobar".to_string()
    }
}

#[cfg(target_os="windows")]
fn win_greet(who: &'static str) {
    println!("Greetings, {}", who );
}


fn greet(who: &'static str) {
    println!("Greetings, {}", who );
}

fn greet_both(first: SString, second: SString) {
    greet(first);
    greet(second);
}

fn get_two()->(i32,i32) {
    (1,2)
}

fn cube(num: f32)->f32 {
    num * num * num
}

/// Entry point
fn main() {
    let pair_of_nums = Pair {one: 1, two: 2};
    let pair_of_strings = Pair { one: "one", two: "foo"};

    let list_of_nums: Vec<&'static str> = vec!["one", "two", "three"];

    println!("{}", pair_of_strings.to_string());
    for item in list_of_nums.iter() {
    };

    let mut x = 0;
    loop {
        x += 1;
        println!("Hello: {}", x);
        if x > 10 {break;};
    }

    for i in 1..10 {
        println!("Aloha: {}", i );
    }

    greet_both("Travis", "Joe");
}

// ,----[ square ]
// | gets the square of a number
// `------------------
fn square(num: i32)->i32 {
    num * num
}

// +------------+
// | Unit Tests |
// +------------+
#[test]
fn test_cube() {
    assert_eq!(cube(2.0), 8.0);
}

#[test]
fn test_square() {
    assert_eq!(square(5), 25);
}

#[test]
fn test_get_two() {
    let (x,y) = get_two();
    assert_eq!(x, 1);
    assert_eq!(y, 2);
}
