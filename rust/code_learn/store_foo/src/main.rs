struct FOO
{
	f: i32,
}
struct Link<'a>{
	link: &'a FOO,
}
fn store_foo<'a>(x: &mut Link<'a>, y: &'a FOO)
{
	x.link=y;
}
fn main() {
    let a=FOO{f: 30};
	let x=&mut Link{link: &a};
	{
		let b=FOO{f: 12};
		store_foo(x,&b);
	}
	println!("{}",x.link.f);
}
