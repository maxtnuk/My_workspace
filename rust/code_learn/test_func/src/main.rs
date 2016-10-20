fn Average_test(x: i32,y: i32) ->i32{
	let sum =x+y;
	sum/2
}
fn main() {
    println!("Hello, average");
	let x=2; 
	let y=2;
	let average =Average_test(x,y);
	println!("answer is {}",average);
}
