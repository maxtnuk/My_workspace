struct Any<'a>
{
	a: &'a mut i32,
	b: &'a mut i32,
}
struct Point<'a>{
	x: &'a mut Any<'a>,
	y: &'a mut Any<'a>,
}
fn main() {

    let mut x =&mut Point{x: &mut Any{a: &mut 10 ,b: &mut 20},y: &mut Any{a: &mut 20,b: &mut 28}};
	{
		let mut y=&mut x;
		let mut z=&mut y;
		*(***z).x.a=11;
		*(***z).x.b=21;
		*(***z).y.a=100;
		*(***z).y.b=10;
		println!("{},{}",*z.x.a,*z.y.b);
	}
	println!("{},{}",*x.x.a,*x.y.b);
}