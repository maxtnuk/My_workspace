struct point<'a>{
	x: &'a i32,
}
fn main() {
	let temp=&20;
    let mut a=point{x: temp};
	let x=&a;
	{
		let y=&x;
		println!("{}",y.x);
	}
	println!("{}",a.x);
	println!("{}",x.x);
}
