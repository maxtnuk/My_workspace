fn main() {
    let a=[-1,-1,3];
    let b=a.into_iter().skip_while(|x| **x<0).cloned().collect::<Vec<i32>>();
    println!("{:?}",b);
}
