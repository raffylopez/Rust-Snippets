#[allow(dead_code)]
#[derive(Debug)]
struct Thing {
    id: i32,
}

impl std::fmt::Display for Thing {
    fn fmt(&self, f: &mut std::fmt::Formatter)->std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[allow(dead_code)]
struct Person<'a> {      // explicit lifetime def
    name: &'static str,  // tie struct assigned to name
                         // to lifetime of Person struct,
                         // so it doesn't get freed while
                         // Person struct hasn't been freed yet
    thing: &'a mut Thing,
}

#[allow(unused_variables)]
#[allow(unused_must_use)]

fn main() {
    let mut thing = &mut Thing {id: 123};
    // borrow immutably
    {
        let y = &thing;
        let t = &thing;  // can borrow mutably multiple times
    } // return borrowed

    // borrow mutably
    {
        let z = &mut thing;
    } // return borrowed

    { // let a Person borrow the thing
        let p = Person {name: "John", thing: &mut thing};
    }
    println!("{}", thing);

    // move ownership completely
    let new_management = thing;
    {
        // move ownership to a variable that only exists in a block
        let naughty_management = new_management;
    } // .. but struct gets freed up here!

    let thing = &mut Thing {id: 123}; // Rust lets you shadow vars


}
