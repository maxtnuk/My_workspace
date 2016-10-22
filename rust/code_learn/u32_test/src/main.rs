
struct Point{
    x: i32,
    y: i32,
}
fn test_function(x:&mut Point){
    x.x+=1;
    x.y+=1;
}
fn main() {
    let x: u32=1_000_000;
    assert_eq!(1000000,x);
}
