fn main() {
   let x=[1,2,3];
   let y=[1;3];
   let read_x= &x[..];
   let read_y= &y[0..1];
   println!("{}",read_x[0]);
   println!("{}",read_y[0]);
}
