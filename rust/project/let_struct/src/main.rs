use std::cell::Cell;

struct foo
{
    x: i32,
    y: Cell<i32>, 
}
fn main() {
    let y=foo{x: 30,y: Cell::new(6)};
    y.y.set(7);
    println!("{}",y.y.get());

}
