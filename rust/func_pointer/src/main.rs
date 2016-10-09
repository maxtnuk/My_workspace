fn add_sum(x: i32,y: i32) -> i32
{
	println!("hello fucker");
	x+y
}

fn main() {
    let f=add_sum;
	let d=add_sum(2,3);
	f(2,3);
	d;
	println!("{}",f(3,4));
	println!("{}",d);
}
