extern crate adder;

#[test]
fn it_works() {
    let a= adder::Add!(2);
    assert_eq!(4,a);
}