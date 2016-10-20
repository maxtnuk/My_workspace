macro_rules! o_O {
    {
        $(
            $x:expr; [$($y:expr),*]
        ),*

    }=> {
        &[$($($x+$y),*),* ]
    }
}
fn main() {
    let a: &[i32] =o_O!(10;[1,2,3],
                       20;[2,3,4]);

    assert_eq!(a,[11,12,13,22,23,24]);
}
