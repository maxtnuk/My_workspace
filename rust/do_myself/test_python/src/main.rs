#![feature(plugin)]
#![plugin(rust_mixin)]

rust_mixin! {r#"
fn main() {
    println!("fn fib_0() -> i32 {{ 0 }}");
    println!("fn fib_1() -> i32 {{ 1 }}");

    for i in 2..(40 + 1) {
        println!("fn fib_{}() -> i32 {{ fib_{}() + fib_{}() }}",
                 i, i - 1, i - 2);
    }
}
"#}

fn main() {
    println!("the 30th fibonacci number is {}", fib_30());
}
