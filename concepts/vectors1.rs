// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

struct MyBox<T> { content: T }
struct Chocolate { brand: &'static str }
trait Chocolatey {
    fn yum();
}

impl Chocolatey for MyBox<Chocolate> {
    fn yum() { println!("yummy!");}
    // fn yum(s: &str) { }
}

fn main() {
    let fruits = ["apple", "grapes"];
    let fruits_as_vec: Vec<_> = fruits.iter().collect();
    let res0 = get_fruits();

    println!("res0: {:?}", res0);

    let c = "carbonadium";
    let mut strvar = String::with_capacity(24);
    strvar.push_str("flamboyant");
    let view = &c[3..6];
    let ssv = &strvar[4..10];
     
    println!("view: {}", ssv);
     
    let cc: Vec<char> = vec!['f','o','o'];

    let elements: Vec<&'static str> = vec!["carbon", "lithium", "strontium"];
    let joined = elements.join("-");
    println!("joined: {:?}", joined);

    let empty_tuple = ();

    let one = (123,456);
    let two = (123,456);

    println!("one == two: {}", one == two);
    let choco_box: MyBox<Chocolate> = MyBox {content: Chocolate {brand:"hershey's"}};
    MyBox::yum();
}

fn get_fruits()-> Vec<&'static str> {
    let f:Vec<_> = vec!["apple", "grapes", "banana"];
    f
}
