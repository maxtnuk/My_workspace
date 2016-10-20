struct Point       
{
    x: i32,
}
fn main() {
    let mut number =Point{x: 20};
    {
        let mut add_num = move |t: i32| {
            number.x +=t};
            add_num(5);
    }
    assert_eq!(20,number.x);
}
