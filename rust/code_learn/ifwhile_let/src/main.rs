
fn main() {
    let mut v =vec![1,2,3,4,11];
    while let Some(element) = v.pop() {
        println!("{}",element);
    }
}
