fn main() {
  	for i in 0..10{
		println!("{}",i);
	}
	for (index,value) in (4..10).enumerate(){
		println!("index = {} and value = {}",index,value);
	}
}
