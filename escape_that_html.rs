// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

fn escape_that_html(ref s:String)->String {
    let mut buf = String::with_capacity(24);
    for c in s.chars() {
        match c {
            '<' => buf.push_str("&lt;"),
            '>' => buf.push_str("&gt;"),
            cc => buf.push(cc),
        }
    }
    buf
}

fn main() {
    let html = "<html><head><title>foo</title></head><body><body>Hey, world!</body>";
    let result = escape_that_html(html.to_string());
    println!("result: {}", result);
}

#[test]
fn it_works() {
    let html = "<html><head><title>foo</title></head><body><body>Hey, world!</body>";
    let expected = "&lt;html&gt;&lt;head&gt;&lt;title&gt;foo&lt;/title&gt;&lt;/head&gt;&lt;body&gt;&lt;body&gt;Hey, world!&lt;/body&gt;";

    println!("expected: {}", expected);
    assert_eq!(escape_that_html(html.to_string()), expected);
}

