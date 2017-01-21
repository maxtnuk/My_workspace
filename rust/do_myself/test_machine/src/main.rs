extern crate rusty_machine;

use rusty_machine::linalg::Matrix;
use rusty_machine::learning::gmm::{CovOption, GaussianMixtureModel};
use rusty_machine::learning::UnSupModel;
fn main() {
    let inputs = Matrix::new(4, 2, vec![1.0, 2.0, -3.0, -3.0, 0.1, 1.5, -5.0, -2.5]);
    let test_inputs = Matrix::new(3, 2, vec![1.0, 2.0, 3.0, 2.9, -4.4, -2.5]);

    // Create gmm with k(=2) classes.
    let mut model = GaussianMixtureModel::new(2);
    model.set_max_iters(10);
    model.cov_option = CovOption::Diagonal;

    // Where inputs is a Matrix with features in columns.
    model.train(&inputs).unwrap();
    // Print the means and covariances of the GMM
    // println!("{:?}", model.means());
    println!("{:?}", model.covariances());

    // Where test_inputs is a Matrix with features in columns.
    let post_probs = model.predict(&test_inputs).unwrap();

    // Probabilities that each point comes from each Gaussian.
    println!("{:?}", post_probs.data());

}
