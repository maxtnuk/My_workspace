fn main() {
    let x: u32 =5;
	println!("our number is {}",x);
	{
		let mut x=x;
		loop
		{
			println!("and now {}",x);
			x=x-1;
			if x==0
			{
				break;
			}
		}
	}
	println!("but our number is fine {}",x);
}
