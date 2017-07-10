

fn main() {
    let nums: Vec<_> = vec![1,2,3,4];
    let handle: Vec<_> = nums.into_iter().map(|e| e * 2).collect();
    println!("{:?}", handle );
}
