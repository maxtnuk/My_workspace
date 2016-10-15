fn main() {
    let x  =5;
    {
        let x =10;
        println!("i am {}",x);
    }
    println!("i am {}",x);
}
