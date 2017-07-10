use std::thread;

fn main() {
    let handle = thread::spawn(|| println!("foo"));
    handle.join().unwrap()
}
