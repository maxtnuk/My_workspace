use std::cell::Cell;

fn main() {
    let c=Cell::new(10);
	c=4;
	//c.set(20);
	println!("{}",c.get());
}
