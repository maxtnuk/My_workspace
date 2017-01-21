extern crate machine_dodge;
use machine_dodge::background::{self, object};
fn main() {
    let mut machine = object::Object::new(20.0, 20.0);
    machine.set_speed(50.0);
    machine.set_color([0.7, 1.0, 0.6, 0.2]);
    let mut env = background::Environment::new();
    env.start(&mut machine);
}
