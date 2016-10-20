struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

trait HasArea { 
     fn area(&self) -> f64; 
}


impl HasArea for Circle {
    fn area(&self) -> f64{
        std::f64::consts::PI * (self.radius * self.radius)
    }
}
fn print_area<T: HasArea>(shape: &T)
{
    println!("shape have the area {}",shape.area());
}

fn main() {
    let i_am_circle = Circle{x:0.0,y:0.0,radius:20.0};
    print_area(&i_am_circle);
    println!("{}",i_am_circle.area());
}
