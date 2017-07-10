#[allow(dead_code)]
#[derive(Debug)]
struct Department<'a> {
    name: &'static str,
    employees: &'a mut Vec<&'static str>
}

#[allow(unused_variables)]
#[allow(unused_must_use)]
fn main() {
    let mut growable: &mut Vec<_> = &mut vec!["foo", "bar"];
    growable.push("baz");
    let dept = Department {name: "Sales", employees: &mut growable};
    // let x = &mut growable;
    // let result: Vec<_> = growable.into_iter().map(|e| { println!("{}", e)}).collect();

    // growable.push("far"); // error, can't even access vec from original
    
    dept.employees.push("far");

    println!("{:?}", dept );
}
