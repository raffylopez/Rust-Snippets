// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

fn do_things() {
}

#[derive(Debug)]
struct Book<'a> {
    title: &'a str, 
    author: Vec<&'a str>,
    pages: i32,
    price_in_dollars: f32
}

struct Shelf<'b> {
    books: Vec<Book<'b>>
}

fn get_input(msg: String)->String {
    println!("{}", msg);
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf);
    buf.trim().to_string()
}

fn get_and_parse(msg: &str)->i32 {
    println!("{}", msg);
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf);

    // match buf.parse() {
    //     Ok(n)=>n,
    //     Err(_)=>0
    // }


    if let Ok(n) = buf.parse() {
        n
    } else {
        0
    }
}


fn main() {
    let ba = Book { title: "Strange Days", pages: 235, price_in_dollars: 10.00, author: vec![]};
    let bb = Book { 
        title: "Castle on the Rockz", 
        pages: 500, 
        price_in_dollars: 20.00, 
        author: vec!["Fleur Nimitz", "Chistel von Heim"]
    };

    // let books = vec![ba,bb];
    //
    // for b in books.iter() {
    //     println!("b: {:?}", b);
    // }

    let s = Shelf{books: vec![ba,bb]};

    let value = get_and_parse("Input please? ");
}
