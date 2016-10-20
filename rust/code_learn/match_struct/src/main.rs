struct Point
{
    x: i32,
    y: i32,
}
fn main() {
    let point =Point {x: 2,y: 3};
    let temp=point.x;
    let x =6;
    match point{
        Point {y,..} => println!("y is {}",y),
    }
    match x {
        e @ 1 ... 5 => println!("hello"),
        _ => {}
    }
}