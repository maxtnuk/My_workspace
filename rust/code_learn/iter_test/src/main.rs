fn main() {
    let a=[1,2,3,4,5];
    let mut x=a.iter();
    let mut y=x.next()
    assert_eq!(1,*x.next().unwrap());
    assert_eq!(1,*a.iter().next().unwrap());
    assert_eq!(2,*x.next().unwrap());
    assert_eq!(2,*a.iter().next().unwrap());
}
