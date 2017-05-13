extern crate whiteread;
use std::io;
use whiteread::WhiteReader;
fn main() {
    let i = io::stdin();
    let mut i = WhiteReader::new(i.lock());
    let (a, b, c): (i32, String, f32) = i.parse().unwrap();
    println!("{},{},{}", a + 10, b, c);
}
