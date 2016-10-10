struct HasDrop;

impl Drop for HasDrop{
    fn drop(&mut self){
        println!("Dropping fuck!! my body lost!!");
    }
}
fn main() {
    let x =HasDrop;
    {
        let y=HasDrop;
        println!("i live in this here");
    }
    println!("hey what are you doing? yeh you lost your body bye bye");
}
