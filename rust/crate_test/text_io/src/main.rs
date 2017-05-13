#[macro_use]
extern crate text_io;
fn main() {
    let (i, f): (i32, f64);
    scan!("{}, {}", i, f);
    let a: u32 = read!("{}\n");
    println!("{} {} enter and {} here", i + 11, f - 20.0, a + 10);
}
