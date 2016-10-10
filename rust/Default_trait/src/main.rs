struct UseDefault;

trait Foo{
    fn is_valid(&self) -> bool;
    fn is_invalid(&self) -> bool {!self.is_valid()}
}

impl Foo for UseDefault{
    fn is_valid(&self) -> bool{
        println!("Called UseDefault.is_valid.");
        true
    }
}

struct OverrideDefault;

impl Foo for OverrideDefault{
    fn is_valid(&self) -> bool{
        println!("Called OverrideDefault.is_valid.");
        true
    }
    fn is_invalid(&self) -> bool{
        println!("Called OverrideDefault.is_invalid.");
        true
    }
}
fn main() {
    let default = UseDefault;
    assert!(!default.is_invalid());
    let over = OverrideDefault;
    assert!(!over.is_invalid());
}
