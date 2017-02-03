extern crate carboxyl;

use std::cell::RefCell;

fn main() {
    let sink = carboxyl::Sink::new();
    let stream = sink.stream();
    let test = stream.map(|x| x * x).hold(0);
    println!("{}", test.sample());
    sink.send(4);
    println!("{}", test.sample());
    let stream2 = sink.stream();
    let mut event = stream2.events();
    sink.send(10);
    println!("event :{},signal :{}", event.next().unwrap(), test.sample());
    println!("{}", test.sample());
    let a = RefCell::new(5);
    let b = a.borrow_mut();
    let c = a.borrow_mut();
    println!("b: {:?},c: {:?}", b, c);
}
