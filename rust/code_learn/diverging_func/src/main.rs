fn diverges()
{
	println!("fuck you");
	panic!();
}
fn return_i(x: i32) -> i32
{
	println!("hello there");
	if x==40
	{
		x
	}
	else
	{
		println!("oh god are you kidding me");
		diverges();
	}
}
fn main() {
   	println!("start");
	let x: i32=return_i(42);
	x;
	println!("end");
}
