extern crate rain;
use rain::Graph;
macro_rules! error {
    ($x:expr) => {{$x.expect("error graph")}};
}
fn main() {
    // Get a drawing area
    let mut graph = Graph::new();

    // Get some line identifiers
    let l1 = "Line 1";
    let l2 = "Line 2";
    let l3 = "Line 3";

    // Add some values and print
    assert!(graph.add(l1, 0).is_ok());
    assert!(graph.add(l2, 0).is_ok());
    error!(graph.print());

    // Add more values and print
    assert!(graph.add(l2, 5).is_ok());
    assert!(graph.add(l3, 10).is_ok());
    error!(graph.print());

    // Remove a line and print
    assert!(graph.remove(l1).is_ok());
    error!(graph.print());

}
