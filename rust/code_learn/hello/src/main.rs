trait ShowPoint {
    fn point_print(&self);
}
#[derive(Debug)]
struct Point{
    x: i32,
    y: i32,
}

impl ShowPoint for Point{
    fn point_print(&self){
        println!("({},{})",self.x,self.y);
    }
}
fn print_to_me(x: Point)-> Point
{
    x.point_print();
    let y =Point{x: 28,y: 17};
    y
}
fn main() {
    let x=vec![2;3];
    let y=x.clone();
    let hello=Point{x: 0,y:1};
    let bye=print_to_me(hello);
    let fuck={
        let sj= Point{x: 218,y: 28};
        sj
    };
    println!("{:?}",bye);
    println!("{} {}",x[0],y[0]);
}
