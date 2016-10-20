fn main() {
    let x =1;
    let c ='c';

    match c{
        x => println!("x: {} c: {}",x,c),//x is fucking shadowed
    }

    println!("x: {}",x);
}
